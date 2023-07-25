use crate::config::Config;
use crate::constants::OptionType;

// Calculates profit (total updated stock value - premium)
fn calc_profit(opt: &OptionType, config: &Config, stock_price: f32) -> f32 {
    let value = 100.0 * config.num * config.strike;
    let premium = 100.0 * config.num * config.contract;
    let updated_value = 100.0 * config.num * stock_price;

    let diff = match opt {
        OptionType::Call => updated_value - value,
        OptionType::Put => value - updated_value,
    };

    diff - premium
}

// Prints valuation table
pub fn print_table(opt: OptionType, config: &Config) {
    for i in 1..10 {
        let percent = (i * 10) as f32 / 100.0;
        let stock_price = match opt {
            OptionType::Call => config.strike * (1.0 + percent),
            OptionType::Put => config.strike * percent,
        };

        println!(
            "${:.2}\t\t| ${:.2}",
            stock_price,
            calc_profit(&opt, &config, stock_price)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_profit_returns_correct_amount_for_call_50() {
        let opt = OptionType::Call;
        let config = Config {
            num: 1.0,
            strike: 50.0,
            contract: 4.85,
        };

        assert_eq!(calc_profit(&opt, &config, 55.0), 15.0);
    }

    #[test]
    fn calc_profit_returns_correct_amount_for_call_120() {
        let opt = OptionType::Call;
        let config = Config {
            num: 1.0,
            strike: 120.0,
            contract: 3.15,
        };

        assert_eq!(calc_profit(&opt, &config, 140.0), 1685.0);
    }

    #[test]
    fn calc_profit_returns_correct_amount_for_put_70() {
        let opt = OptionType::Put;
        let config = Config {
            num: 1.0,
            strike: 70.0,
            contract: 1.44,
        };

        assert_eq!(calc_profit(&opt, &config, 24.0), 4456.0);
    }

    #[test]
    fn calc_profit_returns_correct_amount_for_put_150() {
        let opt = OptionType::Put;
        let config = Config {
            num: 1.0,
            strike: 150.0,
            contract: 40.45,
        };

        assert_eq!(calc_profit(&opt, &config, 100.0), 955.0);
    }
}
