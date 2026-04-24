mod arg;
mod notes;
fn main() {
    let configdata = notes::config::Config::default();
    let storage_init = notes::storage::DataBaseStorage::init(&configdata.storage).unwrap();    
    println!("{:#?}",storage_init);
    arg::arg_setup();
}