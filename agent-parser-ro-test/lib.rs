use agent_parser_ro::{DeviceType, UserAgentParser};
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::{Action, LogLevel};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(AgentParserRoot)
    });
}}

struct AgentParserRoot;

impl Context for AgentParserRoot {}

impl RootContext for AgentParserRoot {
    fn get_type(&self) -> Option<proxy_wasm::types::ContextType> {
        Some(proxy_wasm::types::ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(AgentParserContext))
    }
}

struct AgentParserContext;

impl Context for AgentParserContext {}

impl HttpContext for AgentParserContext {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        // Extract User-Agent header
        let user_agent = self
            .get_http_request_header("user-agent")
            .unwrap_or_default();

        // Detect device type
        let device_type = detect_device(&user_agent);

       println!("Detected device type: {}", device_type);

        // Set device type as nginx log field
        self.set_property(
            vec!["nginx", "log_field1"],
            Some(device_type.as_bytes()),
        );


        Action::Continue
    }
}

fn detect_device(user_agent: &str) -> &'static str {
    let info = UserAgentParser::parse(user_agent);
    match info.device_type {
        DeviceType::Mobile => "mobile",
        DeviceType::Tablet => "tablet",
        DeviceType::Desktop => "pc",
        DeviceType::TV | DeviceType::Game => "appliance",
        DeviceType::Bot => "bot",
        DeviceType::Smartwatch | DeviceType::VRHeadset | DeviceType::CarSystem => "other",
        DeviceType::Unknown => "unknown",
    }
}
