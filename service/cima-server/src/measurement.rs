use anyhow::Error;
use cctrusted_base::tcg;
use openssl::hash::Hasher;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

use crate::{
    agent::IMR,
    cima_pb::{TcgDigest, TcgEventlog},
    policy::PolicyConfig,
};

#[derive(Clone)]
pub struct Measurement {
    policy: PolicyConfig,
    imr: TcgDigest,
    event_logs: Vec<TcgEventlog>,
}

impl Measurement {
    pub fn new(policy: PolicyConfig) -> Measurement {
        let algo_id: u32 = match policy.hash_alogrithm() {
            Some(v) => match v.as_str() {
                "sha1" => tcg::TPM_ALG_SHA1.into(),
                "sha256" => tcg::TPM_ALG_SHA256.into(),
                "sha384" => tcg::TPM_ALG_SHA384.into(),
                "sha512" => tcg::TPM_ALG_SHA512.into(),
                _ => panic!("Unknown hashing algorithm."),
            },
            None => tcg::TPM_ALG_SHA384.into(),
        };

        let algo_len = tcg::TcgDigest::get_digest_size_from_algorithm_id(algo_id as u16);
        let hash = vec![0; algo_len.into()];

        Measurement {
            policy,
            imr: TcgDigest { algo_id, hash },
            event_logs: vec![],
        }
    }

    pub fn imr(&self) -> &TcgDigest {
        &self.imr
    }

    pub fn event_logs(&self) -> &Vec<TcgEventlog> {
        self.event_logs.as_ref()
    }

    fn extend_imr(&mut self, val: &[u8]) -> Result<(), Error> {
        let mut hasher = match Hasher::new(self.imr.clone().into()) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        };
        match hasher.update(val) {
            Ok(_v) => _v,
            Err(e) => return Err(e.into()),
        };
        let hash_val = match hasher.finish() {
            Ok(v) => v.to_vec(),
            Err(e) => return Err(e.into()),
        };

        let new_val = [self.imr.hash.clone(), hash_val.to_vec()].concat();
        match hasher.update(&new_val) {
            Ok(_v) => _v,
            Err(e) => return Err(e.into()),
        };
        self.imr.hash = match hasher.finish() {
            Ok(v) => v.to_vec(),
            Err(e) => return Err(e.into()),
        };

        let digests: Vec<TcgDigest> = vec![TcgDigest {
            algo_id: self.imr.algo_id,
            hash: hash_val.to_vec(),
        }];
        let eventlog = TcgEventlog {
            rec_num: 0,
            imr_index: IMR::CONTAINER as u32,
            event_type: tcg::IMA_MEASUREMENT_EVENT,
            event_size: val.len().try_into().unwrap(),
            event: val.to_vec(),
            digests,
            extra_info: HashMap::new(),
        };

        self.event_logs.push(eventlog);

        Ok(())
    }

    fn get_processes(&mut self, procfs: String) -> Result<HashMap<String, String>, Error> {
        let mut processes = HashMap::new();
        let pattern = Regex::new(r".*/[0-9]").unwrap();
        let paths = fs::read_dir(procfs)
            .unwrap()
            .map(|r| r.unwrap().path())
            .filter(|r| pattern.is_match(r.to_str().unwrap()));

        for path in paths {
            let cmdline_path = path.to_str().unwrap().to_owned() + "/cmdline";
            let cmdline =
                fs::read_to_string(cmdline_path).expect("Failed to read process cmdline.");

            if cmdline.is_empty() {
                continue;
            }

            let (name, parameter) = cmdline.split_once('\0').unwrap();
            processes.insert(name.to_string(), parameter.to_string());
        }

        Ok(processes)
    }

    fn measure_system(&mut self) -> Result<(), Error> {
        let processes = self.get_processes("/proc".to_string()).unwrap();
        let process_policy = match self.policy.system_processes() {
            Some(v) => v.clone(),
            None => return Ok(()),
        };

        for p in &process_policy {
            if processes.contains_key(p) {
                let proc = match self.policy.system_with_parameter() {
                    Some(v) => match v {
                        true => format!("{}\0{}", p, processes[p]),
                        false => p.clone(),
                    },
                    None => p.clone(),
                };
                let _ = self.extend_imr(proc.as_bytes());
            }
        }

        Ok(())
    }

    pub fn measure(&mut self) -> Result<(), Error> {
        self.measure_system()
    }

    pub fn container_isolated(&mut self) -> bool {
        self.policy.container_isolated().unwrap_or(false)
    }
}
