use std::collections::HashSet;

use clap::Parser;

#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf,
}
fn main(){
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).expect("Problem in reading file");
    let mut inp: Vec<Vec<char>> = vec![];
    for line in content.lines(){
        let line = line.replace('.'," ");
        let mut arr = vec![];
        for c in line.chars(){
            arr.push(c);
        }
        inp.push(arr);
    }
    let mut ans = 0;
    let directions: [[i32; 2]; 8] = [[1,0],[0,1],[0,-1],[-1,0],[1,1],[1,-1],[-1,1],[-1,-1]];
    let mut set: HashSet<String> = HashSet::new();
    for row in 0..inp.len(){
        for col in 0..inp[0].len(){
            if !(inp[row][col]=='*'){
                continue
            }
            let mut arr = vec![];
            for dir in directions{
                let nr = row as i32+dir[0];
                let mut nc = col as i32 +dir[1];
                if nr>-1 && nr<(inp.len() as i32) && nc>-1 && nc<(inp[0].len() as i32) && !set.contains(format!("{} {}",nr,nc).as_str()) && inp[nr as usize][nc as usize].is_digit(10){
                    let mut str = String::from("");
                    while nc>-1 && nc<(inp[0].len() as i32) && inp[nr as usize][nc as usize].is_digit(10){
                        nc-=1;
                    }
                    // println!("nr {} nc {}" ,nr,nc);
                    if nc==-1 || !(inp[nr as usize][nc as usize].is_digit(10)){nc+=1}
                    while (nc as usize)<inp[0].len() && inp[nr as usize][nc as usize].is_digit(10){
                        str += String::from(inp[nr as usize][nc as usize]).as_str();
                        set.insert(format!("{} {}",nr,nc));
                        nc+=1;
                    }
                    arr.push(str);
                }
            }
            // println!("arr -> {:?} set - > {:?}",arr,set);
            if arr.len()==2{
                ans += arr[0].parse::<i32>().expect("parse multiply error 1")*arr[1].parse::<i32>().expect("parse multiply error 2");
            }
        }
    }
    println!("ans -> {}",ans);
}
fn first_main(){
    fn is_symbol(c:char) -> bool{
        return !c.is_alphabetic() && !c.is_numeric() && !c.is_whitespace();
    }
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).expect("Problem in reading file");
    let mut inp: Vec<Vec<char>> = vec![];
    for line in content.lines(){
        let line = line.replace('.'," ");
        let mut arr = vec![];
        for c in line.chars(){
            arr.push(c);
        }
        inp.push(arr);
    }
    println!("{:?}",inp);
    let mut ans = 0;
    let directions = [[1,0],[0,1],[0,-1],[-1,0],[1,1],[1,-1],[-1,1],[-1,-1]];
    for row in 0..inp.len(){
        let mut col = 0;
        while col<inp[0].len(){
            let mut str = String::from("");
            let mut accepted = false;
            while col<inp[0].len() && inp[row][col].is_digit(10){
                str+=format!("{}",inp[row][col]).as_str();
                if !accepted{
                    for dir in directions{
                        let nr = (row as i32) +dir[0];
                        let nc = (col as i32) +dir[1];
                        if nr>-1 && nr<(inp.len() as i32) && nc>-1 && nc<(inp[0].len() as i32) && is_symbol(inp[nr as usize][nc as usize]){
                            // let c = inp[nr as usize][nc as usize];
                            //println!("nr ->{} nc -> {} c->{}",nr,nc,c);
                            accepted = true;
                        }
                    } 
                }
                col+=1;
            }
            if accepted{
                //println!("str -> {}",str);
                ans += str.parse::<i32>().expect(format!("can't parse string to int {}",str).as_str());
            }
            col+=1;
        }
    }
    println!("ans -> {}",ans);
}