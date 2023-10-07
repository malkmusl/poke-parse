use std::env;

use parser::stats_parser::{parse_stats, STATS};
use tokio;

use crate::parser::stats_parser::{parse_ev, EVS};

pub mod parser;
pub mod r#struct;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--stats")) {
        parse_stats().await;
        parse_ev().await;

        let base_stats = STATS.lock().unwrap().len();
        let ev_stats = EVS.lock().unwrap().len();

        println!("BaseStats Size: {}", base_stats);
        println!("EvStats Size: {}", ev_stats);
    }
}
