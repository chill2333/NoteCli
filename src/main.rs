mod arg;
mod notes;
fn main() {
    let configdata = notes::config::Config::from_file(Some("./src/config.toml")).unwrap();
    println!("{:#?}",configdata);
    arg::arg_setup();
}