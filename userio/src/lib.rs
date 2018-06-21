extern crate regex;

use regex::Regex;
use std::io;
use std::io::prelude::*;

pub struct PromptUserInfo<'a> {
    pub address_msg: &'a str,
    pub prompt_msg: &'a str,
    pub user_response_validation_pattern: &'a str,
    pub err_msg: &'a str,
}

pub fn prompt_user(info: PromptUserInfo) -> String {
    if info.address_msg != "" {
        println!("{}", info.address_msg);
    }    
    println!("{}", info.prompt_msg);
    
    let stdin = io::stdin();
    let regex = Regex::new(info.user_response_validation_pattern).unwrap();
    let mut line = String::new();
    while line.is_empty() {
        stdin.lock().read_line(&mut line).unwrap();
        line.pop();
        if !line.is_empty() && !regex.is_match(&line) {
            line.clear();
            println!("{}", info.err_msg);
        }
    }
    line 
}
