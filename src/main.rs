use std::fs::File;
use std::io::prelude::*;

fn main() {
    let data = &file_handler();
    println!("{}",data);
    get_head(data);
    
}

fn file_handler() -> String {
    let mut file = File::open("data.csv").expect("Cannot open file");
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
            for val in &value{
                println!("{}",val);
            }
            println!("length[{}]", &value.len());
        }
    }
}
