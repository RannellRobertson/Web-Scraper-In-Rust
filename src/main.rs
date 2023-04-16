use std::{collections::HashMap};
use text_colorizer::*;
use std::path::Path;
use regex::Regex;
use reqwest;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Spider {
    name: String,
    response: Response,

}

#[derive(Debug, Clone)]
struct Response(HashMap<String, String>);

#[derive(Debug)]
struct Arguments {
    name:     String,
    url:      String,
    filename: String,
}

fn print_usage() {
    eprintln!("{} - scrapes, stores and parses the contents of a specific website", "WEB SCRAPER".green());
    eprintln!("Usage: web-scraper <name> <url> <filename> <pattern>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4 got, {}", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        name:        args[0].clone(),
        url:         args[1].clone(), 
        filename:    args[2].clone()
    }
}

impl Spider {
    fn new(name: String) -> Spider {
        Spider {
            name: name,
            response: Response(HashMap::new()),
        }
    }
    
    async fn start_requests(mut self, url: String) -> Response {
        let resp = reqwest::get(&url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        self.response.0.insert(self.name.to_string(), resp);

        self.response
        
    }

    fn parse(self, filename: String, pattern: &str) {
        let data = match fs::read_to_string(&filename) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), filename, e);
                std::process::exit(1);
            }
        };

        let binding = Regex::new(&pattern).unwrap();
        let matches: Vec<&str> = binding.find_iter(&data).map(|x| x.as_str()).collect();
        println!("{:?}", matches);
        
        // match rgx.find(&data) {
        //     Some(x) => println!("{:?}: {:?}", x, rgx.find_iter(&data).map(|mat| mat.as_str())),
        //     None    => unreachable!()
        // }
    }

    fn save_response(self, filename: String, resp: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        for (_, value) in resp {
            let path = Path::new(&filename);
            
            match fs::write(&path, &value) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{} Failed to write to file '{}': {:?}", "ERROR".red().bold(), filename, e); 
                    std::process::exit(1);
                }
            };

        }
        Ok(())
        
    }

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example URLs:
    // 1.) https://example.com:          cargo run example-spider https://example.com example.html
    // 2.) https://books.tosccrape.com:  cargo run bookstoscrape-spider https://books.tosccrape.com bookstoscrape.html 

    let args = parse_args();
    // println!("{:?}", args);

    let spider = Spider::new(args.name);
    let spider2 = Spider::new(String::from("Parse-Spider"));
    
    let resp = spider.clone().start_requests(args.url).await.0;    
    spider.save_response(String::from(&args.filename), &resp);

    // The second spider's purpose is to parse the 
    // results from the first spider.
    // Attempted Regex patterns:

    // /<div>(.*?)<\/div>/g: []
    // <p\s*.*>\s*.*</p>:    ["<p><a href=\"https://www.iana.org/domains/example\">More information...</a></p>"]
    // ^<p>.*?</p>$:         []
    // .*?</p>:              [
    //                        "    domain in literature without prior coordination or asking for permission.</p>", 
    //                        "    <p><a href=\"https://www.iana.org/domains/example\">More information...</a></p>"
    //                       ]      
    spider2.parse(args.filename, r".*?</p>");
        
    Ok(())
}