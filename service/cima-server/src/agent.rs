use anyhow::{anyhow, Error};
use cctrusted_base::{api::CCTrustedApi, api_data::ExtraArgs, tcg};
use cctrusted_vm::sdk::API;
use log::info;
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::{
    cima_pb::{TcgDigest, TcgEventlog},
    container::Container,
    measurement::Measurement,
    policy::PolicyConfig,
};

pub enum IMR {
    FIRMWARE = 0,
    KERNEL = 1,
    SYSTEM = 2,
    CONTAINER = 3,
}

pub struct Agent {
    measurement: Option<Measurement>,
    containers: HashMap<String, Container>,
    event_logs: Vec<TcgEventlog>,
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            measurement: None,
            containers: HashMap::new(),
            event_logs: vec![],
        }
    }

    pub fn init(&mut self, policy: PolicyConfig) -> Result<(), Error> {
        // Measure the system when Agent initialization
        self.measurement = Some(Measurement::new(policy));
        match self
            .measurement
            .as_mut()
            .expect("The measurement was not initialized.")
            .measure()
        {
            Ok(_) => info!("The system has been measured as the policy defined."),
            Err(e) => return Err(e),
        }

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

    fn fetch_all_event_logs(&mut self) -> Result<(), Error> {
        let start: u32 = self.event_logs.len() as u32;

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

                    if tcg_event.event_type == tcg::IMA_MEASUREMENT_EVENT {
                        match self.filter_container(tcg_event.clone()) {
                            Ok(_v) => _v,
                            Err(e) => return Err(e),
                        }
                    }

                    self.event_logs.push(tcg_event)
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
                    self.event_logs.push(TcgEventlog {
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
        info!("Loaded {} event logs.", self.event_logs.len());

        Ok(())
    }

    fn filter_container(&mut self, event: TcgEventlog) -> Result<(), Error> {
        let data = match String::from_utf8(event.event.to_vec()) {
            Ok(v) => v,
            Err(e) => return Err(anyhow!("Convert IMA event data to string failed: {:?}", e)),
        };

        let cgpath: Vec<&str> = data.split(' ').collect();
        if cgpath.len() != 4 {
            return Ok(());
        }

        if cgpath[1].contains("kubepods.slice") || cgpath[1].contains("system.slice/docker-") {
            let container_id = match Container::parse_id(cgpath) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            if !self.containers.contains_key(&container_id) {
                let measurement = match self.measurement.as_mut() {
                    Some(v) => v,
                    None => return Err(anyhow!("The measurement was not initialized.")),
                };

                let mut container =
                    Container::new(measurement.imr().clone(), measurement.event_logs().to_vec());
                match container.extend_imr(IMR::CONTAINER as u32, event.clone()) {
                    Ok(_v) => {
                        self.containers.insert(container_id.clone(), container);
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            } else {
                let container = match self.containers.get_mut(&container_id) {
                    Some(v) => v,
                    None => return Err(anyhow!("Cannot get container as mutable.")),
                };
                return container.extend_imr(IMR::CONTAINER as u32, event.clone());
            }
        }

        Ok(())
    }

    pub fn get_cc_eventlog(
        &mut self,
        container_id: String,
        start: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<TcgEventlog>, Error> {
        let _ = self.fetch_all_event_logs();
        let mut event_logs = vec![];

        let measurement = match self.measurement.as_mut() {
            Some(v) => v,
            None => return Err(anyhow!("The measurement was not initialized.")),
        };

        if measurement.container_isolated() {
            if !self.containers.contains_key(&container_id) {
                return Err(anyhow!("Container cannot be found."));
            }

            for event_log in &self.event_logs {
                if event_log.imr_index == IMR::FIRMWARE as u32
                    || event_log.imr_index == IMR::KERNEL as u32
                {
                    event_logs.push(event_log.clone());
                }
            }

            let container = &self.containers[&container_id];
            event_logs.extend(container.event_logs().clone());
        } else {
            event_logs.extend(self.event_logs.to_vec());
        }

        let begin = match start {
            Some(s) => match s.cmp(&(event_logs.len() as u32)) {
                Ordering::Greater => {
                    return Err(anyhow!(
                        "Invalid input start. Current number of eventlog is {}",
                        event_logs.len()
                    ));
                }
                Ordering::Equal => return Ok(Vec::new()),
                Ordering::Less => s,
            },
            None => 0,
        };

        let end = match count {
            Some(c) => {
                if c == 0 {
                    return Err(anyhow!(
                        "Invalid input count. count must be number larger than 0!"
                    ));
                } else if c + begin > event_logs.len() as u32 {
                    event_logs.len()
                } else {
                    (c + begin).try_into().unwrap()
                }
            }
            None => event_logs.len(),
        };

        Ok(event_logs[begin as usize..end as usize].to_vec())
    }

    pub fn get_cc_report(
        &mut self,
        container_id: String,
        nonce: Option<String>,
        user_data: Option<String>,
    ) -> Result<(Vec<u8>, i32), Error> {
        let _ = self.fetch_all_event_logs();

        let measurement = match self.measurement.as_mut() {
            Some(v) => v,
            None => return Err(anyhow!("The measurement was not initialized.")),
        };

        let new_nonce = if measurement.container_isolated() {
            if !self.containers.contains_key(&container_id) {
                return Err(anyhow!("Container cannot be found."));
            }

            let container = &self.containers[&container_id];
            match nonce {
                Some(v) => match base64::decode(v) {
                    Ok(v) => Some(base64::encode([container.imr().hash.to_vec(), v].concat())),
                    Err(e) => return Err(anyhow!("nonce is not base64 encoded: {:?}", e)),
                },
                None => None,
            }
        } else {
            nonce.clone()
        };

        let (report, cc_type) = match API::get_cc_report(new_nonce, user_data, ExtraArgs {}) {
            Ok(v) => (v.cc_report, v.cc_type as i32),
            Err(e) => return Err(e),
        };

        Ok((report, cc_type))
    }

    pub fn get_cc_measurement(
        &mut self,
        container_id: String,
        index: u32,
        algo_id: u32,
    ) -> Result<TcgDigest, Error> {
        let _ = self.fetch_all_event_logs();

        let measurement = match self.measurement.as_mut() {
            Some(v) => v,
            None => return Err(anyhow!("The measurement was not initialized.")),
        };

        if measurement.container_isolated() {
            if !self.containers.contains_key(&container_id) {
                return Err(anyhow!("Container cannot be found."));
            }

            if index == IMR::SYSTEM as u32 {
                return Err(anyhow!("Cannot access IMR according to the policy."));
            }

            if index == IMR::CONTAINER as u32 {
                let container = match self.containers.get_mut(&container_id) {
                    Some(v) => v,
                    None => {
                        return Err(anyhow!("The container is on the list but fails to get it."))
                    }
                };
                return Ok(container.imr().clone());
            }
        }

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
