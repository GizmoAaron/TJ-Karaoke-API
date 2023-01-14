extern crate reqwest; // 0.11.13
extern crate scraper;
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
use std::fs::File;
use std::io::{stdin,stdout};
use std::io::{Error, Write};
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::BufReader;
use std::time::{Duration, Instant};
use scraper::ElementRef;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;


fn main() -> std::io::Result<()> {
    let start = Instant::now();
    download();
    let duration = start.elapsed();
    println!("This took:{}ms",duration.as_millis());
    return Ok(());
}
fn get_input() -> String{
    let mut s=String::new();
    print!("Please enter a search term: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}
fn download() -> std::io::Result<()> {
    let mut i = 0;
    let file = File::create("songs.csv")?;
    let mut file = LineWriter::new(file);
    let locker = Mutex::new(file);
    let arc_lock = Arc::new(locker);
    let arc_copy = arc_lock.clone();
    (1..200).into_par_iter().for_each(|i|{
        let songs = scrape(i);
        let mut file_handle = arc_copy.lock().unwrap();
        for rowData in songs {
            let mut joined = rowData.join(";");
            joined.push('\n');
            file_handle.write(joined.as_bytes());
        }
    });
    return Ok(());
}
fn scrape(i:i32) -> Vec<Vec<String>>{
    let response = reqwest::blocking::get(
        format!("http://www.tjmedia.co.kr/tjsong/song_search_list.asp?strType=16&strText=0&strCond=0&searchOrderItem=&searchOrderType=&strSize05=100&intPage={}",i)
    )
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);
    let TR  = scraper::Selector::parse("tr").unwrap();
    let TD  = scraper::Selector::parse("td").unwrap();
    let title_selector = scraper::Selector::parse("table.board_type1").unwrap();
    let titles = document.select(&title_selector).next().unwrap();
    let mut my_vec: Vec<Vec<String>> = Vec::new();
    for row in titles.select(&TR){
        let entries = row.select(&TD)
        .map(|val| val.inner_html())
        .collect::<Vec<_>>();
        my_vec.push(entries);
    }
    return my_vec;
}
fn get_api() -> std::io::Result<()> {
    //read data from file
    let file = File::open("songs.csv")?;
    let reader = BufReader::new(file);
    let pat = get_input().to_lowercase();
    let mut song_Data: Vec<String> = Vec::new();
    //start timer
    let start = Instant::now();
    for line in reader.lines() {
        song_Data.push(line?.to_lowercase());
    }
    //println!("Songs:{}",song_Data.len());
    //search for substring and return new list of matches
    let matched_Songs: Vec<String> = song_Data
    .iter()
    .filter(|&s| {
        s.contains(&pat)
    })
    .map(|x| x.clone())
    .collect();
    //print vector
    //println!("Matches:{}",matched_Songs.len());
    let mut table = Table::new();
    table.add_row(row!["Song_ID","Title","Artist","Lyricist","Writer"]);
    matched_Songs
    .iter()
    .for_each(|f| {
        let cells = f.split(';')
        .map(|cell| Cell::new(cell))
        .collect();
        table.add_row(Row::new(cells));
    });
    table.printstd();
    //print duration
    let duration = start.elapsed();
    println!("This took:{}ms",duration.as_millis());
    return Ok(());
}