use cctrusted_base::api::*;
use cctrusted_base::tcg::EventLogEntry;
use cctrusted_ccnp::sdk::API;
use log::*;

fn main() {
    // set log level
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // retrieve cc eventlog with API "get_cc_eventlog"
    info!("call cc trusted API [get_cc_eventlog] to get container related eventlog without count!");
    let eventlogs1 = match API::get_cc_eventlog(Some(0), None) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting eventlog: {:?}", e);
            return;
        }
    };

    info!("container event log count: {}", eventlogs1.len());

    // retrieve cc eventlog with API "get_cc_eventlog"
    info!("call cc trusted API [get_cc_eventlog] to get container related eventlog with count number!");
    let eventlogs = match API::get_cc_eventlog(Some(0), Some(101)) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting eventlog: {:?}", e);
            return;
        }
    };

    info!("event log count: {}", eventlogs.len());
    // for eventlog in &eventlogs {
    //     eventlog.show();
    // }

    // retrieve cc eventlog in batch
    info!("call cc trusted API [get_cc_eventlog] to get container related eventlog in batch size of 10!");
    let mut eventlogs2: Vec<EventLogEntry> = Vec::new();
    let mut start = 0;
    let batch_size = 10;
    loop {
        let event_logs = match API::get_cc_eventlog(Some(start), Some(batch_size)) {
            Ok(q) => q,
            Err(e) => {
                error!("error get eventlog: {:?}", e);
                return;
            }
        };
        for event_log in &event_logs {
            eventlogs2.push(event_log.clone());
        }
        if !event_logs.is_empty() {
            start += event_logs.len() as u32;
        } else {
            break;
        }
    }

    info!("event log count: {}", eventlogs2.len());

    // replay cc eventlog with API "replay_cc_eventlog"
    info!("call cc trusted API [replay_cc_eventlog] to replay container related eventlog!");
    let replay_results = match API::replay_cc_eventlog(eventlogs2) {
        Ok(q) => q,
        Err(e) => {
            error!("error replay eventlog: {:?}", e);
            return;
        }
    };

    // show replay results
    for replay_result in replay_results {
        replay_result.show();
    }
}
