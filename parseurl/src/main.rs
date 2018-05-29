extern crate url;
extern crate time;

use url::{Url, Host};
use std::fs::File;

use std::io::Read;


fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

fn main() {
    // --snip--
    let filename = "urls.txt";

    let urls: Vec<String> = lines_from_file(filename);

    // for link in urls {

        let _issue_list_url = Url::parse("http://банки.рф")
            .expect("Error while handlding the issue_list_url");


        // println!("scheme {}", issue_list_url.scheme()); // https
        // println!("username {:?}", issue_list_url.username()); // ''
        // println!("password {:?}", issue_list_url.password()); // None
        println!("hoststr {:?}", _issue_list_url.host_str()); // Some("github.com")
        // println!("host {:?}", issue_list_url.host()); // Some(Host::Domain("github.com")
        // println!("port {:?}", issue_list_url.port()); // None
        // println!("path {}", issue_list_url.path()); // "/rust-lang/rust/issues"
        // println!("path segments {:?}", issue_list_url.path_segments().map(|c| c.collect::<Vec<_>>())); // Some(vec!["rust-lang", "rust", "issues"])
        // println!("query {:?}", issue_list_url.query()); // Some("labels=E-easy&state=open")
        // println!("fragment {:?}", issue_list_url.fragment()); // None
        // // println!(!issue_list_url.cannot_be_a_base());

    // }
}
