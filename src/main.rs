use colored::Colorize;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => match make_request(&args[1]) {
            Ok(()) => println!("Done without errors"),
            Err(e) => eprintln!("{}", e)
        },
        _ => {
            usage(); 
            panic!("Provide valid IP address or keyword: me.")
        }
    }
}

fn usage() {
    println!("Print usage");
}

fn make_request(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create URL
    let start = "http://ip-api.com/json/";
    let params = "?fields=17032913";

    
    let mut url = format!("{}{}{}", start, ip, params);
    url = url.replace("/me", "");

    // Make request
    let resp = reqwest::blocking::get(url)?.text()?;
    let d: serde_json::Value = serde_json::from_str(&resp)?;
    
    let proxy = match d["proxy"].as_bool().unwrap() {
        true => "PROXY".green(),
        false => "PROXY".red()
    };
    
    let mobile = match d["mobile"].as_bool().unwrap() {
        true => "mobile".green(),
        false => "mobile".red()
    };

    let hosting = match d["hosting"].as_bool().unwrap() {
        true => "hosting".green(),
        false => "hosting".red()
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
    println!(" {} | {} | {} ", proxy, mobile, hosting);
    Ok(())
}

//use json::parse;
//use serde_json::Value;
//use std::collections::HashMap;
//use std::env::args;
//use std::error::Error;
//use reqwest::blocking::get;
//
//// Dont care about pre-request IP validity.
//// Will use API generated errors to handle any error.
//fn main() {
//    let arg: Vec<String> = args().collect();
//    
//    if arg.len() == 1 {
//        usage(); 
//        panic!("Input error. Provide IP address or me keyword")
//    } else {
//        match make_request(&arg[1]) {
//            // Ok(r) => println!("{:#}", parse(&r).unwrap()),
//            Ok(r) => match pprint(r) {},
//            Err(e) => eprintln!("{}", e)
//        }
//    }
//}
//
//fn usage() {
//    println!("Header placeholder.");
//    println!("  arg Argument description");
//}
//
//fn make_request(ip: &str) -> Result<String, Box<dyn Error>> {
//    // Create URL
//    let start = "http://ip-api.com/json/";
//    let params = "?fields=17033177";
//
//    
//    let mut url = format!("{}{}{}", start, ip, params);
//    url = url.replace("/me", "");
//
//    // Make request
//    let response = get(url)?.text()?;
//    Ok(response)
//}
//
//fn pprint(data: String) -> Result<(), Box<dyn Error>> {
//    // let data_json = parse(&data).unwrap();
//    let d: Vec<HashMap<String, Value>> = serde_json::from_str(&data)?;
//    for item in d.iter() {
//        dbg!(item.keys());
//    }
//    Ok(())
//}


