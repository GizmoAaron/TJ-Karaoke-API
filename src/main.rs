#[macro_use] 
extern crate prettytable;
extern crate reqwest; // 0.11.13
extern crate scraper;
use std::io::{stdin,stdout};
use std::io::{Write};
pub mod gather;

fn main() {
    gather::get_api(&get_input());
}
fn get_input() -> String{
    let mut s=String::new();
    print!("Please enter a search term: ");
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}