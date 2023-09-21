// IPID by sm0lt4nk - make request to ip-api.com to fetch data about IP.
use chrono::{Timelike, Utc};
use colored::Colorize;
use reqwest::blocking::get;
use std::env::args;
use std::error::Error;

fn main() {
    let args: Vec<String> = args().collect();
   
    // Make request & print formatted data.
    // Panic & print help message if no ip address were provided.
    match args.len() {
        2 => match make_request(&args[1]) {
            Ok(()) => {
                let t = Utc::now();
                let fmt_time = format!("{}:{}:{}", t.hour(), t.minute(), t.second());
                println!("Finished at {} UTC without errors", fmt_time.bold())
            },
            Err(e) => eprintln!("{}", e)
        },
        _ => {
            usage(); 
            panic!("Provide valid IP address or keyword: me.")
        }
    }
}

// Print help message.
fn usage() {
    println!("IPID - check ID of IP. This script make request to ip-api.com.");
    println!("Usage: ipid ip_address");
    println!("  ip_address      Valid IP address or keyword 'me'.");
}

// Take string response, serde & print formatted data.
fn print_results(data: &str) -> Result<(), Box<dyn Error>> {
    println!("{} {}", "~~~~IP ID~~~~".bold(), "by sm0lt4nk");
    
    let d: serde_json::Value = serde_json::from_str(&data)?;
    
    let proxy = match d["proxy"].as_bool().unwrap() {
        true => "PROXY".green(),
        false => "PROXY".red()
    };
    
    let mobile = match d["mobile"].as_bool().unwrap() {
        true => "MOBILE".green(),
        false => "MOBILE".red()
    };

    let hosting = match d["hosting"].as_bool().unwrap() {
        true => "HOSTING".green(),
        false => "HOSTING".red()
    
    };
    
    println!("IP: {} by: {} ({})", 
             d["query"].as_str().unwrap().bold(), 
             d["isp"].as_str().unwrap().bold(), 
             d["org"].as_str().unwrap()
             );
    
    println!("Location: {}, {} ({}, {})", 
             d["city"].as_str().unwrap().bold(), 
             d["country"].as_str().unwrap().bold(), 
             d["lat"], d["lon"]);
    
    println!("{} | {} | {}", proxy, mobile, hosting);
    
    Ok(())
}

// Make API request.
fn make_request(ip: &str) -> Result<(), Box<dyn Error>> {
    // Create URL
    let start = "http://ip-api.com/json/";
    let params = "?fields=17032913";

    
    let mut url = format!("{}{}{}", start, ip, params);
    url = url.replace("/me", "");

    // Make request
    let resp = get(url)?.text()?;
    print_results(&resp)?;
    Ok(())
}

