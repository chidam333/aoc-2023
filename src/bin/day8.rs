use std::collections::HashMap;

use clap::Parser;
#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf
}
fn main(){
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).unwrap();
    let mut line_it = content.lines();
    let pattern = line_it.next().unwrap().chars().into_iter().collect::<Vec<char>>();
    line_it.next();
    let mut line_map = HashMap::new();
    for line in line_it{
        let line_arr = line.split("=").collect::<Vec<&str>>();
        let openrline = line_arr[1].replace("(", "");
        let closerline = openrline.replace(")", "");
        line_map.insert(line_arr[0].trim(), closerline);
    }
    let mut index = 0;
    let mut cur_key = "AAA"; 
    println!("{:?} {:?}",line_map,pattern);
    while true{
        let curdir = pattern[index%pattern.len()];
        println!("cur_key {} curdir {}",cur_key,curdir);
        let cur_key_arr = line_map.get(cur_key.trim()).unwrap().split(",").collect::<Vec<&str>>();
        if curdir=='L'{            
            cur_key = cur_key_arr[0]; 
            if cur_key_arr[0].trim()=="ZZZ"{
                println!("{}",index+1);
                break;
            }
        }
        if curdir=='R'{
            cur_key = cur_key_arr[cur_key_arr.len()-1];
            if cur_key_arr[cur_key_arr.len()-1].trim()=="ZZZ"{
                println!("{}",index+1);
                break;
            }
        }
        index = index+1;
    }
}