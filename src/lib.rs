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
