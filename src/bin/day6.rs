use std::vec;

use clap::Parser;

#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf
}
fn main(){
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).unwrap();
    let mut lines = content.lines();
    let time = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<i128>().unwrap();
    let distance = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<i128>().unwrap();
    // let distance = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<i32>().unwrap();
    let mut ans_ways = vec![];
    let mut no_ways = 0;
    for start_time in 0..time{
        if start_time*(time-start_time)>distance{
            no_ways+=1;
        }
    }
    ans_ways.push(no_ways);
    let ans = ans_ways.iter().fold(1,|acc,val| acc*val);
    println!("{}",ans);
}