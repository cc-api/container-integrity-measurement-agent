use anyhow::{anyhow, Error};
use cctrusted_base::{api::CCTrustedApi, api_data::ExtraArgs, tcg};
use cctrusted_vm::sdk::API;
use log::info;
use std::collections::HashMap;

use crate::ccnp_pb::{TcgDigest, TcgEventlog};

pub struct Agent {
    pub event_logs: Option<Vec<TcgEventlog>>,
}

impl Agent {
    pub fn init(&mut self) -> Result<(), Error> {
        self.event_logs = Some(vec![]);
        self.fetch_all_event_logs()
    }

    pub fn get_default_algorithm(&mut self) -> Result<u32, Error> {
        let algo = match API::get_default_algorithm() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(algo.algo_id.into())
    }

    pub fn get_measurement_count(&mut self) -> Result<u32, Error> {
        let count = match API::get_measurement_count() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(count.into())
    }

    pub fn fetch_all_event_logs(&mut self) -> Result<(), Error> {
        let start: u32 = self
            .event_logs
            .as_ref()
            .expect("The event_logs is None.")
            .len() as u32;

        let entries = match API::get_cc_eventlog(Some(start), None) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        if entries.is_empty() {
            return Ok(());
        }

        for entry in entries {
            match entry {
                tcg::EventLogEntry::TcgImrEvent(event) => {
                    let mut digests: Vec<TcgDigest> = vec![];
                    for d in event.digests {
                        digests.push(TcgDigest {
                            algo_id: d.algo_id as u32,
                            hash: d.hash,
                        })
                    }
                    let tcg_event = TcgEventlog {
                        rec_num: 0,
                        imr_index: event.imr_index,
                        event_type: event.event_type,
                        event_size: event.event_size,
                        event: event.event,
                        digests,
                        extra_info: HashMap::new(),
                    };

                    self.event_logs
                        .as_mut()
                        .expect("Change eventlog to mut failed.")
                        .push(tcg_event)
                }
                tcg::EventLogEntry::TcgPcClientImrEvent(event) => {
                    let mut digests: Vec<TcgDigest> = vec![];
                    let algo_id = tcg::TcgDigest::get_algorithm_id_from_digest_size(
                        event.digest.len().try_into().unwrap(),
                    );

                    digests.push(TcgDigest {
                        algo_id: algo_id.into(),
                        hash: event.digest.to_vec(),
                    });
                    self.event_logs
                        .as_mut()
                        .expect("Change eventlog to mut failed.")
                        .push(TcgEventlog {
                            rec_num: 0,
                            imr_index: event.imr_index,
                            event_type: event.event_type,
                            event_size: event.event_size,
                            event: event.event,
                            digests,
                            extra_info: HashMap::new(),
                        })
                }
                tcg::EventLogEntry::TcgCanonicalEvent(_event) => {
                    todo!();
                }
            }
        }
        info!(
            "Loaded {} event logs.",
            self.event_logs
                .as_ref()
                .expect("Change eventlog to ref failed.")
                .len()
        );

        Ok(())
    }

    pub fn get_cc_eventlog(&mut self, start: u32, count: u32) -> Result<Vec<TcgEventlog>, Error> {
        let _ = self.fetch_all_event_logs();
        let event_logs = self
            .event_logs
            .as_ref()
            .expect("The eventlog is None.")
            .to_vec();
        let s: usize = start.try_into().unwrap();
        let mut e: usize = (start + count).try_into().unwrap();

        if s >= event_logs.len() {
            return Err(anyhow!(
                "Invalid input start. Start must be smaller than total event log count."
            ));
        }
        if e >= event_logs.len() {
            return Err(anyhow!(
                "Invalid input count. count must be smaller than total event log count."
            ));
        }
        if e == 0 {
            e = event_logs.len();
        }

        Ok(event_logs[s..e].to_vec().clone())
    }

    pub fn get_cc_report(
        &mut self,
        nonce: String,
        user_data: String,
    ) -> Result<(Vec<u8>, i32), Error> {
        let (report, cc_type) = match API::get_cc_report(Some(nonce), Some(user_data), ExtraArgs {})
        {
            Ok(v) => (v.cc_report, v.cc_type as i32),
            Err(e) => return Err(e),
        };

        Ok((report, cc_type))
    }

    pub fn get_cc_measurement(&mut self, index: u32, algo_id: u32) -> Result<TcgDigest, Error> {
        let measurement =
            match API::get_cc_measurement(index.try_into().unwrap(), algo_id.try_into().unwrap()) {
                Ok(v) => TcgDigest {
                    algo_id: v.algo_id.into(),
                    hash: v.hash,
                },
                Err(e) => return Err(e),
            };

        Ok(measurement)
    }
}
