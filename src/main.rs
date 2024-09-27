use env_type::types::EnvType;
use std::str::FromStr;

fn main() {
    let env = EnvType::from_str("d").unwrap();

    println!("{:?}", env);
}
