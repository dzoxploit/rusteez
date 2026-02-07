use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();


    let mut count = HashMap::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
       
        let after_bracket = line.split(']').nth(1).unwrap();
        let level = after_bracket.split_whitespace().next().unwrap();
    
        *count.entry(level.to_string()).or_insert(0) += 1;
    }


    let query = lines.next().unwrap();
    let mut q = query.split_whitespace();

    q.next(); // skip "COUNT"
    let target_level = q.next().unwrap();

    let result = count.get(target_level).unwrap_or(&0);
    println!("{}", result);
}