use std::fs;

fn main() {
    let url = "https://www.rust-lang.org/";
    let md_file_output = "./rust.md";

    let body_raw = reqwest::blocking::get(url);
    let body_raw_text = body_raw.unwrap().text();
    let body = body_raw_text.unwrap();

    let md = html2md::parse_html(&body);

    fs::write(md_file_output, md.as_bytes()).expect("Unable to write file");
    println!("Converted markdown has been saved  in {}.", md_file_output);
}
