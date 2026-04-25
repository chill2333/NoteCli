mod arg;
mod notes;
fn main() {
    let config_path = notes::config::default_config_path();
    let configdata = notes::config::Config::from_file(
        Some(config_path.to_string_lossy().to_string().as_str())
    ).unwrap_or_default();
    let mut storage_init = notes::storage::DataBaseStorage::init(&configdata.storage).unwrap();
    let theme = notes::theme::Theme::from_config(&configdata.theme, !configdata.display.color);
    let output = notes::output::Output::new(theme);
    arg::arg_setup(&mut storage_init, &output, &config_path);
}
