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
