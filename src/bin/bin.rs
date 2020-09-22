use ergo_monitoring::MonitoringYmlConfig;

fn main() {
    let config = MonitoringYmlConfig::new("config.yml").expect("todo anyhow");
    ergo_monitoring::run(config);
}

// TODO
// 1. еще более общая конфигурация?
// 2. вызовы бинаря из любой директории не должен фэйлить парсинг пути config.yml
