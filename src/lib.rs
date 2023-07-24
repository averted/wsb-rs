//! # WSB
//! Calculates and prints a spread table of puts and calls,
//! along with their updated valuations at different price points.
//!
//! # Usage
//! ```
//! Usage: wbs-rs <STRIKE_PRICE> <CONTRACT_PRICE> <NUM_CONTRACTS = 1>
//!
//! Arguments:
//!   <STRIKE_PRICE>        Stock strike price.
//!   <CONTRACT_PRICE>      Price of contract.
//!   <NUM_CONTRACTS>       Number of contracts (default 1).
//! ```
//!
//! ## Examples:
//!
//! * Strike price          - $55
//! * Price of contract     - $4.85
//! * Number of contracts   - 3
//!
//! ```
//! $ wsb-rs 55 4.85 3
//!
//!   Strike price: $55
//!   Price per contract: $4.85
//!   Number of contracts: 3
//!   Premium: $1455
//!
//! Price:     Profit:
//! ------------------------------------
//! $5.50     | $13395.00
//! ------------------------------------
//! $11.00    | $11745.00
//! ------------------------------------
//! $16.50    | $10095.00
//! ------------------------------------
//! $22.00    | $8445.00
//! ------------------------------------
//! $27.50    | $6795.00
//! ------------------------------------
//! $33.00    | $5145.00
//! ------------------------------------
//! $38.50    | $3495.00
//! ------------------------------------
//! $44.00    | $1845.00
//! ------------------------------------
//! $49.50    | $195.00
//! ------------------------------------
//!  $55
//! ------------------------------------
//! $60.50    | $195.00
//! ------------------------------------
//! $66.00    | $1845.00
//! ------------------------------------
//! $71.50    | $3495.00
//! ------------------------------------
//! $77.00    | $5145.00
//! ------------------------------------
//! $82.50    | $6795.00
//! ------------------------------------
//! $88.00    | $8445.00
//! ------------------------------------
//! $93.50    | $10095.00
//! ------------------------------------
//! $99.00    | $11745.00
//! ------------------------------------
//! $104.50   | $13395.00
//! ------------------------------------
//! ```

pub mod config;
use config::Config;

enum OptionType {
    Call,
    Put,
}

fn print_table(opt_type: OptionType, config: &Config) {
    let premium = config.contract * 100.0 * config.num;
    let constant_value = 100.0 * config.num * config.strike;

    for i in 1..10 {
        let percent = (i * 10) as f32 / 100.0;
        let new_stock_price = match opt_type {
            OptionType::Call => config.strike * percent,
            OptionType::Put => config.strike * (1.0 + percent),
        };

        let updated_value = 100.0 * config.num * new_stock_price;

        let diff = match opt_type {
            OptionType::Call => constant_value - updated_value,
            OptionType::Put => updated_value - constant_value,
        };

        let profit = diff - premium;

        println!("${:.2}\t\t| ${:.2}", new_stock_price, profit);
        println!("------------------------------------");
    }
}

pub fn run(config: Config) {
    println!("");
    println!("  Strike price: ${}", config.strike);
    println!("  Price per contract: ${}", config.contract);
    println!("  Number of contracts: {}", config.num);
    println!("  Premium: ${}", config.contract * 100.0 * config.num);

    println!("");
    println!("Price:\t\t Profit:");
    println!("------------------------------------");

    print_table(OptionType::Call, &config);

    println!(" ${}", config.strike);
    println!("------------------------------------");

    print_table(OptionType::Put, &config);
}
