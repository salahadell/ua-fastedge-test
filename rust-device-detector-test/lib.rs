use lazy_static::lazy_static;
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::{Action, LogLevel};
use rust_device_detector::device_detector::{Detection, DeviceDetector};

lazy_static! {
    static ref DETECTOR: DeviceDetector = DeviceDetector::new();
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(DeviceDetectorRoot)
    });
}}

struct DeviceDetectorRoot;

impl Context for DeviceDetectorRoot {}

impl RootContext for DeviceDetectorRoot {
    fn get_type(&self) -> Option<proxy_wasm::types::ContextType> {
        Some(proxy_wasm::types::ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(DeviceDetectorContext))
    }
}

struct DeviceDetectorContext;

impl Context for DeviceDetectorContext {}

impl HttpContext for DeviceDetectorContext {
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
    match DETECTOR.parse(user_agent, None) {
        Ok(Detection::Known(device)) => {
            if device.is_television() || device.is_console() {
                "appliance"
            } else if device.is_tablet() {
                "tablet"
            } else if device.is_mobile() {
                "mobile"
            } else if device.is_desktop() {
                "pc"
            } else {
                "unknown"
            }
        }
        Ok(Detection::Bot(_)) => "bot",
        Err(_) => "error",
    }
}
