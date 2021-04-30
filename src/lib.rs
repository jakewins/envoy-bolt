
use log::{trace, info};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use proxy_wasm::hostcalls;

mod bolt;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_stream_context(|context_id, _root_context_id| -> Box<dyn StreamContext> {
        Box::new(BoltEnvoyFilter { 
            context_id,
            downstream_parser: bolt::DownstreamParser::new(),
        })
    })
}

struct BoltEnvoyFilter {
    context_id: u32,
    downstream_parser: bolt::DownstreamParser,
}

impl Context for BoltEnvoyFilter {}

impl StreamContext for BoltEnvoyFilter {
    fn on_downstream_data(&mut self, data_size: usize, _end_of_stream: bool) -> Action {
        info!("DATAS: {}", data_size);
        if let Some(b) = self.get_downstream_data(0, data_size) {
            match self.downstream_parser.parse(&b) {
                Ok(_) => (),
                Err(bolt::ParserStatus::Suspend) => (),
                Err(e) => {
                    panic!("{:?}", e);
                }
            }
        } else {
            info!("  No Payload.")
        }
        Action::Continue
    }
}
