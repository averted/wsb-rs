//! ### About
//! Calculates and prints a spread table of puts and calls,
//! along with their updated valuations at different price points.
//!
//! ### Usage:
//! ```ignore
//! $ wbs-rs <STRIKE_PRICE> <CONTRACT_PRICE> <NUM_CONTRACTS = 1>
//!
//! Arguments:
//!   <STRIKE_PRICE>        Stock strike price.
//!   <CONTRACT_PRICE>      Price of contract.
//!   <NUM_CONTRACTS>       Number of contracts.
//! ```

pub mod config;
mod constants;
mod utils;

use config::Config;
use constants::OptionType;

pub fn run(config: Config) {
    println!("");
    println!("  Strike price: ${}", config.strike);
    println!("  Price per contract: ${}", config.contract);
    println!("  Number of contracts: {}", config.num);
    println!("  Premium: ${:.2}", config.contract * 100.0 * config.num);

    println!("");
    println!("Price:\t\t Profit:");
    println!("------------------------------------");

    utils::print_table(OptionType::Put, &config);

    println!("------------------------------------");
    println!(" ${}", config.strike);
    println!("------------------------------------");

    utils::print_table(OptionType::Call, &config);
}
