use log::info;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Starting node");
}
