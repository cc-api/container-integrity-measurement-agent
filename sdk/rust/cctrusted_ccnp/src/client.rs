use crate::client::ccnp_server_pb::{
    ccnp_client::CcnpClient, GetCcEventlogRequest, GetCcEventlogResponse, GetCcMeasurementRequest,
    GetCcMeasurementResponse, GetCcReportRequest, GetCcReportResponse,
};
use cctrusted_base::api_data::ExtraArgs;
use cctrusted_base::cc_type::TeeType;
use core::result::Result::Ok;
use hashbrown::HashMap;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tonic::Request;
use tower::service_fn;

//FixMe: use map from cc_type
lazy_static! {
    pub static ref TEE_VALUE_TYPE_MAP: HashMap<u32, TeeType> = {
        let mut map: HashMap<u32, TeeType> = HashMap::new();
        map.insert(0, TeeType::TPM);
        map.insert(1, TeeType::TDX);
        map.insert(2, TeeType::SEV);
        map.insert(3, TeeType::CCA);
        map
    };
}

pub mod ccnp_server_pb {
    tonic::include_proto!("ccnp_server_pb");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("ccnp_server_descriptor");
}

pub struct CcnpServiceClient {
    pub ccnp_uds_path: String,
}

impl CcnpServiceClient {
    async fn get_cc_report_from_server_async(
        &mut self,
        nonce: Option<String>,
        data: Option<String>,
        _extra_args: ExtraArgs,
    ) -> Result<GetCcReportResponse, anyhow::Error> {
        let uds_path = self.ccnp_uds_path.parse::<Uri>().unwrap();
        let channel = Endpoint::try_from("http://[::]:0")
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                UnixStream::connect(uds_path.to_string())
            }))
            .await
            .unwrap();

        let request = Request::new(GetCcReportRequest {
            nonce: nonce.unwrap(),
            user_data: data.unwrap(),
        });

        let mut ccnp_client = CcnpClient::new(channel);

        let response = ccnp_client
            .get_cc_report(request)
            .await
            .unwrap()
            .into_inner();
        Ok(response)
    }

    // turn async call to sync call
    pub fn get_cc_report_from_server(
        &mut self,
        nonce: Option<String>,
        data: Option<String>,
        extra_args: ExtraArgs,
    ) -> Result<GetCcReportResponse, anyhow::Error> {
        let response = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.get_cc_report_from_server_async(nonce, data, extra_args));
        response
    }

    pub fn get_tee_type_by_value(&self, tee_id: &u32) -> TeeType {
        match TEE_VALUE_TYPE_MAP.get(tee_id) {
            Some(tee_type) => tee_type.clone(),
            None => TeeType::PLAIN,
        }
    }

    async fn get_cc_measurement_from_server_async(
        &mut self,
        index: u8,
        algo_id: u16,
    ) -> Result<GetCcMeasurementResponse, anyhow::Error> {
        let uds_path = self.ccnp_uds_path.parse::<Uri>().unwrap();
        let channel = Endpoint::try_from("http://[::]:0")
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                UnixStream::connect(uds_path.to_string())
            }))
            .await
            .unwrap();

        let request = Request::new(GetCcMeasurementRequest {
            index: index.into(),
            algo_id: algo_id.into(),
        });

        let mut ccnp_client = CcnpClient::new(channel);

        let response = ccnp_client
            .get_cc_measurement(request)
            .await
            .unwrap()
            .into_inner();
        Ok(response)
    }

    // turn async call to sync call
    pub fn get_cc_measurement_from_server(
        &mut self,
        index: u8,
        algo_id: u16,
    ) -> Result<GetCcMeasurementResponse, anyhow::Error> {
        let response = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.get_cc_measurement_from_server_async(index, algo_id));
        response
    }

    async fn get_cc_eventlog_from_server_async(
        &mut self,
        start: Option<u32>,
        count: Option<u32>,
    ) -> Result<GetCcEventlogResponse, anyhow::Error> {
        let uds_path = self.ccnp_uds_path.parse::<Uri>().unwrap();
        let channel = Endpoint::try_from("http://[::]:0")
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                UnixStream::connect(uds_path.to_string())
            }))
            .await
            .unwrap();

        let request = Request::new(GetCcEventlogRequest {
            start: start.unwrap(),
            count: count.unwrap(),
        });

        let mut ccnp_client = CcnpClient::new(channel);

        let response = ccnp_client
            .get_cc_eventlog(request)
            .await
            .unwrap()
            .into_inner();
        Ok(response)
    }

    // turn async call to sync call
    pub fn get_cc_eventlog_from_server(
        &mut self,
        start: Option<u32>,
        count: Option<u32>,
    ) -> Result<GetCcEventlogResponse, anyhow::Error> {
        let response = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.get_cc_eventlog_from_server_async(start, count));
        response
    }
}
