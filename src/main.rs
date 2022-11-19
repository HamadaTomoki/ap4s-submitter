mod enums;

use std::time;

use ap4s::{model::Student, Crowling};

use headless_chrome::{Browser, LaunchOptions};
use text_io::read;

#[allow(unused_must_use)]
fn main() {
    // Crowling mode
    let mut headless = None;
    while headless.is_none() {
        print!("\nDo you use the 'Headless Mode'? [Y/n] ");
        let input: String = read!();
        let option = input.trim().to_uppercase();
        match &*option {
            "Y" => {
                headless = Some(true);
            }
            "N" => {
                headless = Some(false);
            }
            _ => {
                println!("\n-- Pleaase type 'Y' or 'n' here again. --\nYou should type an option that is not in the options.");
            }
        }
    }
    let headless = headless.unwrap();

    // setup headless chrome
    let option = LaunchOptions {
        headless,
        idle_browser_timeout: time::Duration::from_secs(200),
        ..Default::default()
    };
    let browser = Browser::new(option).unwrap();
    let crowling = Crowling::new(headless, browser);

    // Student infomation
    println!("\n-- Please type your student information. --");
    print!("    Class ID: ");
    let class_id: String = read!("{}\n");
    print!("    Student ID: ");
    let id: String = read!("{}\n");
    print!("    Your Name: ");
    let name: String = read!("{}\n");
    let student = Student { class_id, id, name };

    // Google form url
    let mut has_error = true;
    while has_error {
        println!("\n-- Pleaase type a form url here ↓ --");
        print!("    Form URL: ");
        let input: String = read!("{}\n");
        if url::Url::parse(&input).is_ok() {
            has_error = crowling.crowl(&input, &student).is_err();
            if has_error {
                println!("\n【Url is not Valid !】\nYou should type url collectly.");
            } else {
                crowling.close();
            }
        } else {
            println!("\n【Url is not Valid !】\nYou should type url collectly.");
        }
    }

    println!("...Done");
    crowling.close();
}
