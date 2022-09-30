#![deny(warnings)]
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FreightConfig {
    dir: String,
}

impl ::std::default::Default for FreightConfig {
    fn default() -> Self { Self { dir: "tests/support".into() } }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let cfg: FreightConfig = confy::load("freight", None).unwrap();
    let file = confy::get_configuration_file_path("freight", None).unwrap();
    println!("The configuration file path is: {:#?}", file);
    println!("The configuration is:");
    println!("{:#?}", cfg);

    warp::serve(warp::fs::dir(cfg.dir))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
