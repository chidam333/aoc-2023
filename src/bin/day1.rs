use std::collections::HashMap;

use clap::Parser;
#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf,
}
fn main(){
    let args = Cli::parse();
    let mut map = HashMap::new();
    map.extend(
        [
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ].iter().copied()
    );   
    let content =  std::fs::read_to_string(&args.path).expect("couldn't read the file");
    let mut ans = 0;
    for line in content.lines(){
        let mut first_str = (f32::INFINITY,0) ; 
        let mut last_str = (f32::NEG_INFINITY,0);
        for (key,value) in map.iter(){
            if line.contains(key){
                let arr:Vec<usize> = line.match_indices(key).map(|(i, _)| i).collect();
                for ij in arr{
                    let i = ij as i32;
                    if (i as f32)<first_str.0{
                        first_str.0 = i as f32;
                        first_str.1 = *value;
                    }
                    if (i as f32)>last_str.0{
                        last_str.0 = i as f32;
                        last_str.1 = *value;
                    }
                }
            }
        }
        // println!("first is {} last is {}",first_str.0,last_str.0);
        let mut index = 0;
        for i in line.chars(){
            // println!("i {} index {}",i,index);
            if i.is_digit(10) && (index as f32)<first_str.0{
                first_str.0 = index as f32;
                first_str.1 = format!("{}",i).parse::<i32>().expect("parsing to int error");
                break;
            }
            index+=1;
        }
        index = line.len()-1;
        for i in line.chars().rev(){
            if i.is_digit(10) && (index as f32)>last_str.0{
                last_str.0 = index as f32;
                last_str.1 = format!("{}",i).parse::<i32>().expect("parsing to int error");
                break;
            }
            if index>0 {index -=1;}
        }
        let line_ans = format!("{}{}",first_str.1,last_str.1);
        // println!("value - > {} first_index - > {} last_index - > {} len - > {}",line_ans,first_str.0,last_str.0,line.len());
        ans += line_ans.parse::<i32>().expect("faced trouble in String to int conversion");
        // println!("{} {}",line_ans,ans);
    }
    println!("{}",ans);
}