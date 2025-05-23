use contracts::*;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


pub struct KwicSystem{
    pub lines: Vec<String>,
    pub pos_index: HashMap<String,usize>,
}

pub struct KwicResult{
    pub n_line: u32,
    pub left_context: String,
    pub key_Word: String,
    pub right_context: String,
    pub line: String,
}

