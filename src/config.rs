use std::env;

pub struct Config {
    pub num: f32,
    pub strike: f32,
    pub contract: f32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let strike = match args.next() {
            Some(v) => match v.parse::<f32>() {
                Ok(v) => v,
                Err(_) => return Err("Invalid strike price"),
            },
            None => return Err("Missing strike price. \n\nwsb-rs $strike $contract"),
        };

        let contract = match args.next() {
            Some(v) => match v.parse::<f32>() {
                Ok(v) => v,
                Err(_) => return Err("Invalid contract price"),
            },
            None => return Err("Missing contract price. \n\nwsb-rs $strike $contract"),
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
