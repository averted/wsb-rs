pub struct Config {
    pub num: f32,
    pub strike: f32,
    pub contract: f32,
}

impl Config {
    pub fn new<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();

        let strike = match args.next() {
            Some(v) => match v.parse::<f32>() {
                Ok(v) => v,
                Err(_) => return Err("Invalid strike price"),
            },
            None => return Err("Missing strike price. \n\nwsb-rs <STRIKE_PRICE> <CONTRACT_PRICE>"),
        };

        let contract = match args.next() {
            Some(v) => match v.parse::<f32>() {
                Ok(v) => v,
                Err(_) => return Err("Invalid contract price"),
            },
            None => {
                return Err("Missing contract price. \n\nwsb-rs <STRIKE_PRICE> <CONTRACT_PRICE>")
            }
        };

        let num = match args.next() {
            Some(v) => match v.parse::<f32>() {
                Ok(v) => v,
                Err(_) => return Err("Invalid number of contracts"),
            },
            None => 1.0,
        };

        Ok(Self {
            num,
            strike,
            contract,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_is_valid() {
        let args = ["skip", "55", "4.85"].iter().map(|s| s.to_string());
        let cfg = Config::new(args).unwrap();

        assert_eq!(cfg.num, 1.0);
        assert_eq!(cfg.strike, 55.0);
        assert_eq!(cfg.contract, 4.85);
    }

    #[test]
    #[should_panic(expected = "Missing strike price")]
    fn config_new_should_panic_on_missing_strike_price() {
        let args = ["skip"].iter().map(|s| s.to_string());
        let cfg = Config::new(args).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid strike price")]
    fn config_new_should_panic_on_invalid_strike_price() {
        let args = ["skip", "a", "4.85"].iter().map(|s| s.to_string());
        let cfg = Config::new(args).unwrap();
    }

    #[test]
    #[should_panic(expected = "Missing contract price")]
    fn config_new_should_panic_on_missing_contract_price() {
        let args = ["skip", "55"].iter().map(|s| s.to_string());
        let cfg = Config::new(args).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid contract price")]
    fn config_new_should_panic_on_invalid_contract_price() {
        let args = ["skip", "55", "a"].iter().map(|s| s.to_string());
        let cfg = Config::new(args).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid number of contracts")]
    fn config_new_should_panic_on_invalid_num_contracts() {
        let args = ["skip", "55", "4.55", "a"].iter().map(|s| s.to_string());
        let cfg = Config::new(args).unwrap();
    }
}
