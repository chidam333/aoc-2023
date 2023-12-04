use std::collections::{HashSet, HashMap};

use clap::Parser;

#[derive(Parser)]
struct CLi{
    path:std::path::PathBuf,
}
fn main(){
    let args = CLi::parse();
    let content = std::fs::read_to_string(args.path).expect("failed to read the file");
    let lines = content.lines().collect::<Vec<&str>>();
    let mut map = HashMap::new();
    let mut new_map = HashMap::new();
    for i in 0..lines.len(){
        map.insert(i, 1);
        new_map.insert(i, 1);
    }
    let mut ans = 0;
    let mut cur_index = 0;
    while map.get(&(lines.len()-1)).expect("can't find key")!=&0 && cur_index<lines.len(){
        // println!("new_map {:?}",new_map);
        map.insert(cur_index, map.get(&cur_index).expect("some error")-1);
        let mut dup_index = cur_index;
        let line = lines[cur_index];
        if map.get(&cur_index).expect("can't find this index")==&0{
            cur_index+=1;
            if cur_index==lines.len(){
                break;
            }
        }
        // println!(" cur {}",cur_index);
        // println!("cur {} stack {:?}",cur_index,stack);
        let line = line.replace("  ", " ");
        let line = line.split(":").collect::<Vec<&str>>()[1];
        let parts = line.split("|").collect::<Vec<&str>>();
        let winning_part = parts[0].trim().split(" ").map(|n| n.trim().parse::<i32>().expect("parse error 1")).collect::<HashSet<i32>>();
        let your_part = parts[1].trim().split(" ").map(|n| n.trim().parse::<i32>().expect("parse error 2")).collect::<HashSet<i32>>();
        for i in your_part{
            if winning_part.contains(&i){
                dup_index+=1;
                let new_val = map.get(&dup_index).expect("can't find this key")+1;
                // println!("dup_index {} i {} new_val {}",dup_index,i,new_val);
                map.insert(dup_index,new_val);
                new_map.insert(dup_index,new_val);
            }
        }
        ans+=1;
    }
    println!("ans {} map {:?} ",ans+1,new_map);
}