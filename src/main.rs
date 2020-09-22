use ergo_monitoring::{MonitoringService, MonitoringYmlConfig, MonitoringConfig};

fn main() {
    let config = MonitoringYmlConfig::new("config.yml").expect("todo anyhow");
    println!("{:?}", config.get_sources());
    let service = MonitoringService::new(config);
    println!("{:?}", service.run());
}

// TODO
// 1. как сделать конфигурацию более абстрактной?
// 2. типы ошибок
// 3. run from any library

// Main idea
//-> Result<(), Box<dyn std::error::Error>> {
// let a = Url::parse("http://88.198.13.202:9053/info")?;
// let resp = reqwest::blocking::get(a)?
//     .json::<HashMap<String, serde_json::Value>>()?;
// println!("{:?}", resp["peersCount"].as_u64().unwrap());
// Ok(())