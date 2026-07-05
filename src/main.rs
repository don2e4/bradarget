use std::fs;
use std::io;

fn main() {
    let mut url = String::new();
    let mut filename = String::new();

    println!("hello enter url");
        io::stdin()
            .read_line(&mut url)
            .expect("Invalid URL");
        let url = url.trim();
        println!("pls wait bradar. We r fetching {url}");

    let response = reqwest::blocking::get(url)
        .expect("Not a correct URL");

    let bytes = response.bytes()
        .expect("i cant read those bytes bradar");

    println!("enter your filename bradar");
        io::stdin()
            .read_line(&mut filename)
            .expect("Invalid Filename");
        let filename = filename.trim();
        
    fs::write(filename, bytes)
        .expect("Failed to write file");

    println!("u saved file as {filename}");
}
