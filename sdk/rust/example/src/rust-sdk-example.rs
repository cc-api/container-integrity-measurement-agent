use cctrusted_base::api::*;
use cctrusted_base::api_data::*;
use cctrusted_base::cc_type::TeeType;
use cctrusted_base::tcg::EventLogEntry;
use cctrusted_base::tcg::TcgAlgorithmRegistry;
use cctrusted_base::tdx::quote::TdxQuote;
use cima::sdk::API;
use log::*;
use rand::Rng;

fn get_cc_report() {
     /***
     * Note: in real user case, the nonce should come from attestation server
     * side to prevent replay attack and the data should be generate by API caller
     * according to user define spec
     */
    let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
    let data = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

    // retrieve cc report with API "get_cc_report"
    info!("call cc trusted API [get_cc_report] to retrieve cc report with nonce and data!");
    let report = match API::get_cc_report(Some(nonce), Some(data), ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            info!("error getting cc report: {:?}", e);
            std::process::exit(-1);
        }
    };

    info!("length of the cc report: {}", report.cc_report.len());

    // dump the cc report with API "dump_cc_report"
    //info!("call cc trusted API [dump_cc_report] to dump cc report!");
    //API::dump_cc_report(&report.cc_report);

    // parse the cc report with API "parse_cc_report"
    if report.cc_type == TeeType::TDX {
        let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report.cc_report) {
            Ok(q) => q,
            Err(e) => {
                info!("error parse tdx quote: {:?}", e);
                std::process::exit(-1);
            }
        };
        info!(
            "version = {}, report_data = {}",
            tdx_quote.header.version,
            base64::encode(tdx_quote.body.report_data)
        );

        // show data of the struct TdxQuoteHeader
        info!("call struct show function to show data of the struct TdxQuoteHeader!");
        tdx_quote.header.show();
    }

    // retrieve cc report with API "get_cc_report"
    info!("call cc trusted API [get_cc_report] to retrieve cc report with no nonce and data!");
    let report1 = match API::get_cc_report(None, None, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            info!("error getting cc report: {:?}", e);
            std::process::exit(-1);
        }
    };

    info!("length of the cc report: {}", report1.cc_report.len());

    // parse the cc report with API "parse_cc_report"
    if report1.cc_type == TeeType::TDX {
        let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report1.cc_report) {
            Ok(q) => q,
            Err(e) => {
                info!("error parse tdx quote: {:?}", e);
                std::process::exit(-1);
            }
        };
        info!(
            "version = {}, report_data = {}",
            tdx_quote.header.version,
            base64::encode(tdx_quote.body.report_data)
        );

        // show data of the struct TdxQuoteHeader
        info!("call struct show function to show data of the struct TdxQuoteHeader!");
        tdx_quote.header.show();
    }
}

fn get_cc_measurement() {
    // get default algorithm with API "get_default_algorithm"
    info!("call cc trusted API [get_default_algorithm] to get supported algorithm!");
    let defalt_algo = match API::get_default_algorithm() {
        Ok(algorithm) => {
            info!("supported algorithm: {}", algorithm.algo_id_str);
            algorithm
        }
        Err(e) => {
            error!("error get algorithm: {:?}", e);
            std::process::exit(-1);
        }
    };

    // get number of measurement registers
    info!("call cc trusted API [get_measurement_count] to get number of measurement registers!");
    let _count = match API::get_measurement_count() {
        Ok(count) => {
            info!("measurement registers count: {}", count);
            count
        }
        Err(e) => {
            error!("error get measurement count: {:?}", e);
            std::process::exit(-1);
        }
    };

    // retrive and show measurement registers
    info!("call cc trusted API [get_cc_measurement] to get measurement register content!");
    for index in [0, 1, 3] {
        let tcg_digest = match API::get_cc_measurement(index, defalt_algo.algo_id) {
            Ok(tcg_digest) => tcg_digest,
            Err(e) => {
                error!("error get measurement: {:?}", e);
                std::process::exit(-1);
            }
        };
        info!(
            "show index = {}, algo = {:?}, hash = {:02X?}",
            index,
            tcg_digest.get_algorithm_id_str(),
            tcg_digest.get_hash()
        );
    }
}

fn get_cc_eventlog() {
    // retrieve cc eventlog with API "get_cc_eventlog"
    info!("call cc trusted API [get_cc_eventlog] to get container related eventlog without count!");
    let eventlogs1 = match API::get_cc_eventlog(Some(0), None) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting eventlog: {:?}", e);
            std::process::exit(-1);
        }
    };

    info!("container event log count: {}", eventlogs1.len());

    // retrieve cc eventlog with API "get_cc_eventlog"
    info!("call cc trusted API [get_cc_eventlog] to get container related eventlog with count number!");
    let eventlogs = match API::get_cc_eventlog(Some(0), Some(101)) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting eventlog: {:?}", e);
            std::process::exit(-1);
        }
    };

    info!("event log count: {}", eventlogs.len());
    // for eventlog in &eventlogs {
    //     eventlog.show();
    // }

    // retrieve cc eventlog in batch
    info!("call cc trusted API [get_cc_eventlog] to get container related eventlog in 10 batches!");
    let mut eventlogs2: Vec<EventLogEntry> = Vec::new();
    let mut start = 0;
    let batch_size = (eventlogs1.len() / 10) as u32;
    loop {
        info!("batch start: {}", start);
        let event_logs = match API::get_cc_eventlog(Some(start), Some(batch_size)) {
            Ok(q) => q,
            Err(e) => {
                error!("error get eventlog: {:?}", e);
                std::process::exit(-1);
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
    let replay_results = match API::replay_cc_eventlog(eventlogs) {
        Ok(q) => q,
        Err(e) => {
            error!("error replay eventlog: {:?}", e);
            std::process::exit(-1);
        }
    };

    // show replay results
    for replay_result in replay_results {
        replay_result.show();
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    get_cc_report();
    get_cc_measurement();
    get_cc_eventlog();
}
