use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct SystemPolicy {
    with_parameter: Option<bool>,
    processes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct KubernetesPolicy {
    with_parameter: Option<bool>,
    pods: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct ContainerPolicy {
    with_parameter: Option<bool>,
    isolated: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct MeasurePolicy {
    system: Option<SystemPolicy>,
    kubernetes: Option<KubernetesPolicy>,
    container: Option<ContainerPolicy>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PolicyConfig {
    backend: Option<String>,
    hash_algorithm: Option<String>,
    measure: Option<MeasurePolicy>,
}

impl PolicyConfig {
    pub fn new(path: String) -> PolicyConfig {
        let file = std::fs::File::open(path).expect("Failed to open policy file.");
        serde_yaml::from_reader(file).expect("Failed to serialize policy file.")
    }

    pub fn hash_alogrithm(&self) -> Option<&String> {
        self.hash_algorithm.as_ref()
    }

    pub fn system_processes(&self) -> Option<&Vec<String>> {
        match &self.measure {
            Some(v) => match &v.system {
                Some(v) => v.processes.as_ref(),
                None => None,
            },
            None => None,
        }
    }

    pub fn system_with_parameter(&self) -> Option<bool> {
        match &self.measure {
            Some(v) => match &v.system {
                Some(v) => v.with_parameter,
                None => None,
            },
            None => None,
        }
    }

    pub fn container_isolated(&self) -> Option<bool> {
        match &self.measure {
            Some(v) => match &v.container {
                Some(v) => v.isolated,
                None => None,
            },
            None => None,
        }
    }
}
