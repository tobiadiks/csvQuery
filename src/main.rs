use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::{thread, time};

fn main() {
    //let time_sec = time::Duration::new(2, 000000000);
    let app = App::new("csv-query")
        .author("Adeleke Oluwatobi, tobiadiks@gmail.com")
        .version("1.0.0")
        .about("A csv querying utility")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Opens a valid csv file")
                .takes_value(true),
        )
        .subcommand(
            App::new("param")
                .short_flag('p')
                .long_flag("parameter")
                .about("prints the header params of the csv"),
        )
        .subcommand(
            App::new("range")
                .short_flag('r')
                .long_flag("range")
                .about("prints data from range x - y"),
        )
        .get_matches();

    let path = app.value_of("file").expect("This path is wrong");
    let file_path = Path::new(&path);
    let subcommand = app.subcommand();

    if !file_path.exists() {
        if file_path.extension().unwrap() != "csv" {
            println!("unsupported extension");
        } else {
            println!("file does not exist");
        }
    } else {
        println!("Opening {:?}...\n", &file_path);
        //thread::sleep(time_sec);
        let data = &file_handler(&file_path);

        match subcommand {
            Some(("param", _)) => get_head(data),
            Some(("range", _)) => (),
            _ => println!("invalid subcommand, use -h flag to see available commands"),
        }
    }
}

fn file_handler(path: &Path) -> String {
    let mut file = File::open(path).expect("Cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Opps! cannot read file");
    content
}

fn get_head(data: &String) {
    let heads = data.lines();

    for (i, head) in heads.enumerate() {
        if i == 0 {
            let value: Vec<&str> = head.split(',').collect();
            for val in &value {
                println!("{}", val);
            }
            println!("\nlength[{}]", &value.len());
        }

        
    }
}
