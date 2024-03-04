pub mod agent;
pub mod container;
pub mod measurement;
pub mod policy;
pub mod service;
pub mod ccnp_pb {
    tonic::include_proto!("ccnp_server_pb");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("ccnp_server_descriptor");
}

use anyhow::Result;
use clap::Parser;
use log::info;
use std::{fs, os::unix::fs::PermissionsExt};
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

use ccnp_pb::{ccnp_server::CcnpServer, FILE_DESCRIPTOR_SET};
use policy::PolicyConfig;
use service::Service;

#[derive(Parser)]
struct Cli {
    /// UDS sock file
    #[arg(short, long)]
    #[clap(default_value = "/run/ccnp/uds/ccnp-server.sock")]
    sock: String,
    /// Input policy file
    #[arg(short, long)]
    policy: String,
}

fn set_sock_perm(sock: &str) -> Result<()> {
    let mut perms = fs::metadata(sock)?.permissions();
    perms.set_mode(0o666);
    fs::set_permissions(sock, perms)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = Cli::parse();
    let sock = cli.sock;
    let policy = PolicyConfig::new(cli.policy);

    let _ = std::fs::remove_file(sock.clone());
    let uds = match UnixListener::bind(sock.clone()) {
        Ok(r) => r,
        Err(e) => panic!("[ccnp-server]: bind UDS socket error: {:?}", e),
    };
    let uds_stream = UnixListenerStream::new(uds);
    info!("[ccnp-server]: set sock file permissions: {}", sock);
    set_sock_perm(&sock.clone())?;

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<CcnpServer<Service>>().await;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    info!("[ccnp-server]: staring the service...");
    let service = Service::new(policy);
    Server::builder()
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(CcnpServer::new(service))
        .serve_with_incoming(uds_stream)
        .await?;
    Ok(())
}

#[cfg(test)]
mod ccnp_server_test{
    use super::*;
    use serial_test::serial;
    use policy::PolicyConfig;
    use tower::service_fn;
    use rand::Rng;
    use service::Service;
    use std::fs::read_to_string;
    use tokio::net::{UnixListener, UnixStream};
    use tokio_stream::wrappers::UnixListenerStream;
    use tonic::transport::{Server, Endpoint, Uri, Channel};
    use cctrusted_base::{cc_type::TeeType, tcg};
    use ccnp_pb::{ccnp_client::CcnpClient, GetCcReportRequest, GetCcMeasurementRequest, GetCcEventlogRequest};
    use crate::agent::IMR;

    async fn creat_server() {
        let sock = String::from("/tmp/ccnp-server.sock");
        let policy_path = String::from("./configs/policy.yaml");
        let policy = PolicyConfig::new(policy_path);

        let _ = std::fs::remove_file(sock.clone());
        let uds = match UnixListener::bind(sock.clone()) {
            Ok(r) => r,
            Err(e) => panic!("[ccnp-server]: bind UDS socket error: {:?}", e),
        };

        let uds_stream = UnixListenerStream::new(uds);
        assert!(set_sock_perm(&sock.clone()).is_ok(), "set_perm failed");

        let service = Service::new(policy);
        tokio::spawn(async {
            Server::builder()
            .add_service(CcnpServer::new(service))
            .serve_with_incoming(uds_stream)
            .await
            .unwrap();
        });
    }

    async fn create_client() -> CcnpClient<Channel> {
        let channel = Endpoint::try_from("http://[::]:40081")
            .unwrap()
            .connect_with_connector(service_fn(|_: Uri| {
                let path = "/tmp/ccnp-server.sock";
                UnixStream::connect(path)
            }))
            .await
            .unwrap();

        let client = CcnpClient::new(channel);
        return client;
    }

    fn get_container_id() -> String {
        let mountinfo = "/proc/self/mountinfo".to_string();
        let docker_pattern = "/docker/containers/";
        let k8s_pattern = "/kubelet/pods/";

        let data_lines: Vec<String> = read_to_string(mountinfo)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        for line in data_lines {
            if line.contains(docker_pattern){
                let element = line.split(docker_pattern).last();
                if element.is_some() {
                    let (id, _) = element.unwrap().split_once('/').unwrap();
                    return id.to_string();
                } 
            }

            if line.contains(k8s_pattern) {
                let element = line.split(k8s_pattern).last();
                if element.is_some() {
                    let (left, _) = element.unwrap().split_once('/').unwrap();
                    let id = left.replace('-', "_");
                    return id;
                }
            }
        }
        return "".to_string();
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_report_normal() {
        creat_server().await;
        let mut client = create_client().await;
        let user_data = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
        let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

        let container_id = get_container_id();
        assert_ne!(container_id.len(), 0);
        
        let request = tonic::Request::new(GetCcReportRequest {
            container_id: container_id,
            user_data: Some(user_data),
            nonce: Some(nonce),
        });

        let response = client.get_cc_report(request).await.unwrap().into_inner();
        assert_eq!(response.cc_type, TeeType::TDX as i32);
        assert_ne!(response.cc_report.len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_report_empty_container_id() {
        creat_server().await;
        let mut client = create_client().await;
        let user_data = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
        let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

        let request = tonic::Request::new(GetCcReportRequest {
            container_id: "".to_string(),
            user_data: Some(user_data),
            nonce: Some(nonce),
        });

        let result = client.get_cc_report(request).await;
        assert!(result.is_err(), "Excepted an error");
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_report_empty_user_data() {
        creat_server().await;
        let mut client = create_client().await;
        let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

        let container_id = get_container_id();
        assert_ne!(container_id.len(), 0);
        
        let request = tonic::Request::new(GetCcReportRequest {
            container_id: container_id,
            user_data: Some("".to_string()),
            nonce: Some(nonce),
        });

        let response = client.get_cc_report(request).await.unwrap().into_inner();
        assert_eq!(response.cc_type, TeeType::TDX as i32);
        assert_ne!(response.cc_report.len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_report_empty_nonce() {
        creat_server().await;
        let mut client = create_client().await;
        let user_data = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
    
        let container_id = get_container_id();
        assert_ne!(container_id.len(), 0);
        
        let request = tonic::Request::new(GetCcReportRequest {
            container_id: container_id,
            user_data: Some(user_data),
            nonce: Some("".to_string()),
        });

        let response = client.get_cc_report(request).await.unwrap().into_inner();
        assert_eq!(response.cc_type, TeeType::TDX as i32);
        assert_ne!(response.cc_report.len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_measurement_normal() {
        creat_server().await;
        let mut client = create_client().await;

        let container_id = get_container_id();
        assert_ne!(container_id.len(), 0);
        
        let request = tonic::Request::new(GetCcMeasurementRequest {
            container_id: container_id,
            index: IMR::CONTAINER as u32,
            algo_id: tcg::TPM_ALG_SHA384.into(),
        });

        let response = client.get_cc_measurement(request).await.unwrap().into_inner();
        let cc_measurement = response.measurement.unwrap();
        assert_eq!(cc_measurement.algo_id, tcg::TPM_ALG_SHA384.into());
        assert_ne!(cc_measurement.hash.len(), 0)
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_measurement_empty_container_id() {
        creat_server().await;
        let mut client = create_client().await;
        
        let request = tonic::Request::new(GetCcMeasurementRequest {
            container_id: "".to_string(),
            index: IMR::CONTAINER as u32,
            algo_id: tcg::TPM_ALG_SHA384.into(),
        });

        let result = client.get_cc_measurement(request).await;
        assert!(result.is_err(), "Excepted an error");
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_measurement_unexpected_index() {
        creat_server().await;
        let mut client = create_client().await;

        let container_id = get_container_id();
        assert_ne!(container_id.len(),0);
        
        let request = tonic::Request::new(GetCcMeasurementRequest {
            container_id: container_id,
            index: IMR::SYSTEM as u32,
            algo_id: tcg::TPM_ALG_SHA384.into(),
        });

        let result = client.get_cc_measurement(request).await;
        assert!(result.is_err(),"Excepted an error");
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_measurement_unexpected_algorithm() {
        creat_server().await;
        let mut client = create_client().await;

        let container_id = get_container_id();
        assert_ne!(container_id.len(),0);
        
        let request = tonic::Request::new(GetCcMeasurementRequest {
            container_id: container_id,
            index: IMR::CONTAINER as u32,
            algo_id: tcg::TPM_ALG_SHA512.into(),
        });

        let result = client.get_cc_measurement(request).await;
        assert!(result.is_err(), "Excepted an error");
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_eventlog_normal() {
        creat_server().await;
        let mut client = create_client().await;

        let container_id = get_container_id();
        assert_ne!(container_id.len(), 0);
        
        let request = tonic::Request::new(GetCcEventlogRequest {
            container_id: container_id,
            start: Some(0),
            count: Some(1),
        });

        let response = client.get_cc_eventlog(request).await.unwrap().into_inner();
        assert_eq!(response.event_logs.len(), 1);

        let event_log = response.event_logs[0].clone();
        assert_eq!(event_log.event_type, tcg::EV_NO_ACTION.into());
        assert_eq!(event_log.digests[0].algo_id, tcg::TPM_ALG_SHA1.into());
        assert_ne!(event_log.event.len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_eventlog_empty_container_id() {
        creat_server().await;
        let mut client = create_client().await;
        
        let request = tonic::Request::new(GetCcEventlogRequest {
            container_id: "".to_string(),
            start: Some(0),
            count: Some(1),
        });

        let result = client.get_cc_eventlog(request).await;
        assert!(result.is_err(), "Excepted an error");
    }

    #[tokio::test]
    #[serial]
    async fn request_to_cc_eventlog_multiple_count() {
        creat_server().await;
        let mut client = create_client().await;

        let container_id = get_container_id();
        assert_ne!(container_id.len(),0);
        
        let request = tonic::Request::new(GetCcEventlogRequest {
            container_id: container_id,
            start: Some(0),
            count: Some(3),
        });

        let response = client.get_cc_eventlog(request).await.unwrap().into_inner();
        assert_eq!(response.event_logs.len(), 3);
    }
}
