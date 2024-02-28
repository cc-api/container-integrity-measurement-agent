use crate::client::CcnpServiceClient;
use anyhow::*;
use cctrusted_base::api::CCTrustedApi;
use cctrusted_base::api_data::ReplayResult;
use cctrusted_base::api_data::{Algorithm, CcReport, ExtraArgs};
use cctrusted_base::binary_blob::dump_data;
use cctrusted_base::eventlog::EventLogs;
use cctrusted_base::tcg::*;
use core::result::Result::Ok;

const UDS_PATH: &str = "/run/ccnp/uds/ccnp-server.sock";

pub struct API {}

impl CCTrustedApi for API {
    // CCTrustedApi trait function: get cc report from CCNP server
    fn get_cc_report(
        nonce: Option<String>,
        data: Option<String>,
        extra_args: ExtraArgs,
    ) -> Result<CcReport, anyhow::Error> {
        let mut ccnp_service_client = CcnpServiceClient {
            ccnp_uds_path: UDS_PATH.to_string(),
        };

        let response = match ccnp_service_client.get_cc_report_from_server(nonce, data, extra_args)
        {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!("[get_cc_report] err get cc report: {:?}", e));
            }
        };

        Ok(CcReport {
            cc_report: response.cc_report,
            cc_type: ccnp_service_client.get_tee_type_by_value(&response.cc_type),
        })
    }

    // CCTrustedApi trait function: dump report of in hex and char format
    fn dump_cc_report(report: &Vec<u8>) {
        dump_data(report)
    }

    // CCTrustedApi trait function: get max number of IMRs
    fn get_measurement_count() -> Result<u8, anyhow::Error> {
        let mut ccnp_service_client = CcnpServiceClient {
            ccnp_uds_path: UDS_PATH.to_string(),
        };

        let response = match ccnp_service_client.get_cc_measurement_count_from_server() {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!(
                    "[get_measurement_count] err get cc measurement count: {:?}",
                    e
                ));
            }
        };

        Ok(response.count.try_into().unwrap())
    }

    // CCTrustedApi trait function: get measurements
    fn get_cc_measurement(index: u8, algo_id: u16) -> Result<TcgDigest, anyhow::Error> {
        let mut ccnp_service_client = CcnpServiceClient {
            ccnp_uds_path: UDS_PATH.to_string(),
        };

        let response = match ccnp_service_client.get_cc_measurement_from_server(index, algo_id) {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!(
                    "[get_cc_measurement] err get cc measurement: {:?}",
                    e
                ));
            }
        };

        let measurement = match response.measurement {
            Some(measurement) => measurement,
            None => return Err(anyhow!("[get_cc_measurement] faile to get cc measurement")),
        };

        Ok(TcgDigest {
            algo_id: measurement.algo_id as u16,
            hash: measurement.hash,
        })
    }

    // CCTrustedApi trait function: get eventlogs
    fn get_cc_eventlog(
        start: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<EventLogEntry>, anyhow::Error> {
        let mut ccnp_service_client = CcnpServiceClient {
            ccnp_uds_path: UDS_PATH.to_string(),
        };

        let response = match ccnp_service_client.get_cc_eventlog_from_server(start, count) {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!("[get_cc_eventlog] err get cc eventlog: {:?}", e));
            }
        };

        let mut event_logs: Vec<EventLogEntry> = Vec::new();

        for el in response.event_logs {
            if !el.digests.is_empty() {
                let mut digests: Vec<TcgDigest> = Vec::new();
                for d in el.digests {
                    digests.push(TcgDigest {
                        algo_id: d.algo_id as u16,
                        hash: d.hash,
                    });
                }
                event_logs.push(EventLogEntry::TcgImrEvent(TcgImrEvent {
                    imr_index: el.imr_index,
                    event_type: el.event_type,
                    digests: digests.clone(),
                    event_size: el.event_size,
                    event: el.event.clone(),
                }));
            } else {
                event_logs.push(EventLogEntry::TcgPcClientImrEvent(TcgPcClientImrEvent {
                    imr_index: el.imr_index,
                    event_type: el.event_type,
                    digest: el.digests[0].hash[0..20].try_into().unwrap(),
                    event_size: el.event_size,
                    event: el.event.clone(),
                }));
            }
        }

        Ok(event_logs)
    }

    // CCTrustedApi trait function: replay eventlogs
    fn replay_cc_eventlog(
        eventlogs: Vec<EventLogEntry>,
    ) -> Result<Vec<ReplayResult>, anyhow::Error> {
        EventLogs::replay(eventlogs)
    }

    // CCTrustedApi trait function: get default algorithm
    fn get_default_algorithm() -> Result<Algorithm, anyhow::Error> {
        let mut ccnp_service_client = CcnpServiceClient {
            ccnp_uds_path: UDS_PATH.to_string(),
        };

        let response = match ccnp_service_client.get_cc_default_algorithm_from_server() {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!(
                    "[get_default_algorithm] err get cc get default algorithm: {:?}",
                    e
                ));
            }
        };

        let algo_id = response.algo_id as u16;
        Ok(Algorithm {
            algo_id,
            algo_id_str: ALGO_NAME_MAP.get(&algo_id).unwrap().to_owned(),
        })
    }
}
