use crate::ccnp_pb::{TcgDigest, TcgEventlog};
use anyhow::{anyhow, Error};
use cctrusted_base::tcg;
use openssl::hash::{Hasher, MessageDigest};
use regex::Regex;

impl From<TcgDigest> for MessageDigest {
    fn from(digest: TcgDigest) -> Self {
        let algo_id: u16 = digest.algo_id.try_into().unwrap();
        match algo_id {
            tcg::TPM_ALG_SHA1 => MessageDigest::sha1(),
            tcg::TPM_ALG_SHA256 => MessageDigest::sha256(),
            tcg::TPM_ALG_SHA384 => MessageDigest::sha384(),
            _ => MessageDigest::sha256(),
        }
    }
}

pub struct Container {
    imr: TcgDigest,
    event_logs: Vec<TcgEventlog>,
}

impl Container {
    pub fn new(imr: TcgDigest, event_logs: Vec<TcgEventlog>) -> Container {
        Container { imr, event_logs }
    }

    pub fn parse_id(cgpath: Vec<&str>) -> Result<String, Error> {
        if cgpath[1].starts_with("/kubepods") {
            let id_regex1 = Regex::new(r"kubepods-pod|.slice").unwrap();
            let id_regex2 = Regex::new(r"kubepods-besteffort-pod|.slice").unwrap();

            let cgroup: Vec<&str> = cgpath[1].split('/').collect();
            let id = match cgroup.len() {
                4 => id_regex1.replace_all(cgroup[2], "").to_string(),
                5 => id_regex2.replace_all(cgroup[3], "").to_string(),
                _ => {
                    return Err(anyhow!(
                        "The container id parse failed, unknown cgpath format."
                    ))
                }
            };

            Ok(id)
        } else {
            let id_regex = Regex::new(r"/system.slice/docker-|.scope").unwrap();
            let id = id_regex.replace_all(cgpath[1], "").to_string();
            if id.is_empty() {
                return Err(anyhow!("The container id parse failed, id is empty."));
            }

            Ok(id)
        }
    }

    pub fn imr(&self) -> &TcgDigest {
        &self.imr
    }

    pub fn event_logs(&self) -> &Vec<TcgEventlog> {
        self.event_logs.as_ref()
    }

    pub fn extend_imr(&mut self, imr_index: u32, mut event: TcgEventlog) -> Result<(), Error> {
        let digests = event.clone().digests;

        for digest in digests {
            if self.imr.hash.len() != digest.hash.len() {
                return Err(anyhow!("The hash algorithm does not match."));
            }

            let new_val = [self.imr.hash.clone(), digest.hash.clone()].concat();
            let mut hasher = match Hasher::new(digest.clone().into()) {
                Ok(v) => v,
                Err(e) => return Err(e.into()),
            };

            match hasher.update(&new_val) {
                Ok(v) => v,
                Err(e) => return Err(e.into()),
            };

            self.imr.hash = match hasher.finish() {
                Ok(v) => v.to_vec(),
                Err(e) => return Err(e.into()),
            };
        }

        event.imr_index = imr_index;
        self.event_logs.push(event);

        Ok(())
    }
}
