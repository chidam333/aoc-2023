use std::collections::{VecDeque, HashSet, HashMap};

use::clap::Parser;

#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf
}

fn main(){
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).unwrap();
    println!("{}",content);
    let mut curr: i32 = 0;
    let mut curc: i32 = 0;
    let mut dug: Vec<[i32; 2]> = vec![];
    dug.push([curr,curc]);
    for line in content.lines(){
        let mut map = HashMap::new();
        map.insert("0","R");
        map.insert("1","D");
        map.insert("2","L");
        map.insert("3","U");
        let parts = line.split(" ").collect::<Vec<&str>>();
        let colref = parts[2].split("").collect::<Vec<&str>>();
        let mut col:Vec<String>=vec![];
        for i in 3..colref.len()-2{
            col.push(colref[i].to_owned());
        }
        // println!("{:?}",col);
        let dir: &str = map.get(col.pop().unwrap().as_str()).unwrap();
        let mut depth = col.join("");
        let mut depth = i32::from_str_radix(depth.as_str(), 16).unwrap(); 
        if dir=="R"{
            while depth>0{
                curc+=1;
                dug.push([curr,curc]);
                depth-=1;
            }
        }
        if dir=="L"{
            while depth>0{
                // println!("curc {} depth {}",curc,depth);
                curc-=1;
                dug.push([curr,curc]);
                depth-=1;
            }
        }
        if dir=="U"{
            while depth>0{
                curr-=1;
                dug.push([curr,curc]);
                depth-=1;
            }
        }
        if dir=="D"{
            while depth>0{
                curr+=1;
                dug.push([curr,curc]);
                depth-=1;
            }
        }
    }
    let mut maxr = 0;
    let mut maxc = 0;
    fn correct(mut dug_clone:Vec<[i32; 2]>) -> Vec<[i32; 2]>{
        let mut minrow = 0;
        let mut mincol = 0;
        for cord in dug_clone.clone(){
            minrow = std::cmp::min(minrow, cord[0]);
            mincol = std::cmp::min(mincol,cord[1]);
        }
        for cord in &mut dug_clone{
            // println!("{} {}",cord[0],cord[1]);
            cord[0] = cord[0] - minrow;
            cord[1] = cord[1] - mincol;
            // println!("{} {}",cord[0],cord[1]);
        }
        // println!("{:?} minrow {} mincol {}",dug_clone,minrow,mincol);
        return dug_clone;
    }
    dug = correct(dug);
    for i in dug.clone(){
        if i[0]>maxr{maxr=i[0];}
        if i[1]>maxc{maxc=i[1];}
    }
    let mut arr = vec![];
    for r in 0..maxr+1{
        let mut inarr = vec![];
        for c in 0..maxc+1{
            inarr.push('.');
        }
        arr.push(inarr);
        // println!("{:?}",arr);
    } 
    for i in dug.clone(){
        arr[i[0] as usize][i[1] as usize] = '#';
    }
    for row in &arr{
        println!("{:?}",row);
    }
    let mut stack = vec![];
    let mut visited = HashSet::new();
    for i in 0..maxr+1{
        stack.push(format!("{} 0",i));
        stack.push(format!("{} {}",i,maxc));
    }
    for i in 0..maxc+1{
        stack.push(format!("0 {}",i));
        stack.push(format!("{} {}",maxr,i));
    }
    let total = (maxr+1)*(maxc+1);
    let mut count = 0;
    while stack.len()>0{
        let cord = stack.pop().unwrap();
        let parts = cord.split(" ").collect::<Vec<&str>>();
        let row = parts[0].parse::<i32>().unwrap();
        let col = parts[1].parse::<i32>().unwrap();
        let dir = [[1,0],[-1,0],[0,1],[0,-1]];
        // println!("{:?} {:?} {} {} {}",visited,stack,count,row,col);
        if arr[row as usize][col as usize] == '.'{
            for d in dir{
                let nr = row+d[0];
                let nc = col+d[1];
                if nr>-1 && nr<=maxr as i32 && nc>-1 && nc<=maxc as i32 && arr[nr as usize][nc as usize] == '.'{
                    if !visited.contains(&format!("{} {}",nr,nc)){
                        stack.push(format!("{} {}",nr,nc));
                    }
                }
            }
            if !visited.contains(&format!("{} {}",row,col)){count+=1};
        }
        visited.insert(cord.clone());
    }
    println!("{:?}",total-count);
}