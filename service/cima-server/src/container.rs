use crate::cima_pb::{TcgDigest, TcgEventlog};
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
        let path = match cgpath[1].find("kubepods.slice") {
            Some(v) => cgpath[1].get(v..),
            None => match cgpath[1].find("system.slice") {
                Some(v) => cgpath[1].get(v..),
                None => {
                    return Err(anyhow!(
                        "The container id parse failed, system.slice/kubepods.slice not found."
                    ))
                }
            },
        };

        let id = match path {
            Some(v) => match Regex::new(
                r"[[:xdigit:]_]{36}.slice|[[:xdigit:]]{32}.slice|[[:xdigit:]]{64}.scope",
            ) {
                Ok(re) => match re.find(v) {
                    Some(v) => v.as_str().replace(".slice", "").replace(".scope", ""),
                    None => {
                        return Err(anyhow!("The container id parse failed, pattern not found."))
                    }
                },
                Err(_) => {
                    return Err(anyhow!(
                        "The container id parse failed, regex return errors."
                    ))
                }
            },
            None => return Err(anyhow!("The container id parse failed, path is None.")),
        };

        Ok(id.to_string())
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
