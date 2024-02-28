use cctrusted_base::api::*;
use cctrusted_base::api_data::*;
use cctrusted_base::cc_type::TeeType;
use cctrusted_base::tdx::quote::TdxQuote;
use cctrusted_ccnp::sdk::API;
use log::*;
use rand::Rng;

fn main() {
    // set log level
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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
            return;
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
                return;
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
            return;
        }
    };

    info!("length of the cc report: {}", report1.cc_report.len());

    // parse the cc report with API "parse_cc_report"
    if report1.cc_type == TeeType::TDX {
        let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report1.cc_report) {
            Ok(q) => q,
            Err(e) => {
                info!("error parse tdx quote: {:?}", e);
                return;
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
