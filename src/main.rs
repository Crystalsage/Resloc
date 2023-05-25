mod hosts;
mod types;
mod errors;

mod domains;

use domains::domain_to_ascii;

fn main() { 
    domain_to_ascii("http://google.com".to_string(), true);
}
