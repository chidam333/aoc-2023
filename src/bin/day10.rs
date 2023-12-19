use std::collections::{HashMap, HashSet};

use clap::Parser;

#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf
}

fn main(){
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).unwrap();
    let mut arr = vec![];
    let mut row_index = 0;
    let mut start = "".to_owned();
    for line in content.lines(){
        let mut row = vec![];
        let mut col_index = 0;
        let elemenets = line.split("").collect::<Vec<&str>>();
        for ele in 1..elemenets.len()-1{
            if elemenets[ele]=="S" {
                start = format!("{} {}",row_index,col_index);
            }
            row.push(elemenets[ele]);
            col_index+=1;
        }
        arr.push(row);
        row_index+=1;
    }
    let mut map = HashMap::new();
    map.insert("L",[1,1,0,0]);
    map.insert("|",[1,0,0,1]);
    map.insert("-",[0,1,1,0]);
    map.insert("F", [0,1,0,1]);
    map.insert("7",[0,0,1,1]);
    map.insert("J",[1,0,1,0]);
    map.insert("S",[1,1,1,1]);
    struct Node{
        row:i32,
        col:i32,
        count:i32
    }
    impl Clone for Node{
        fn clone(&self) -> Self {
            Node {
                row:self.row,
                col:self.col,
                count:self.count
            }
        }
    }
    let dir = [[-1,0],[0,1],[0,-1],[1,0]];
    let start_parts = start.split(" ").collect::<Vec<&str>>();
    let start_node = Node{
        row:start_parts[0].parse::<i32>().unwrap(),
        col:start_parts[1].parse::<i32>().unwrap(),
        count:0
    };
    let mut stack = vec![start_node];
    let mut pair = HashMap::new();
    let mut visited = HashSet::new();
    pair.insert(0, 3);
    pair.insert(3, 0);
    pair.insert(1, 2);
    pair.insert(2, 1);
    let mut all_nodes = vec![];
    fn combat(side:i32,symbol:&str,pair: HashMap<i32, i32>,map: HashMap<&str, [i32; 4]>,breaker:bool)->bool{
        if breaker {return breaker;}
        let req_side = *pair.get(&side).unwrap();
        let sides_of_req = map.get(symbol).unwrap();
        let breaker = sides_of_req[req_side as usize]==1;
        // println!("searching {}'s viablity and it is {} entered using side {} with requiring side being {}",symbol,breaker,side,req_side);
        return breaker;
    }
    while stack.len()>0{
        // println!("{:?}",visited);
        let top = stack.pop().unwrap();
        let ele = arr[top.row as usize][top.col as usize];
        let finstr = format!("{} {}",top.row,top.col);
        visited.insert(finstr);
        // println!("top ele - > {}",ele);
        let sides = map.get(ele).unwrap();
        for i in 0..dir.len(){
            if sides[i]==0 {continue;}
            let nr = top.row+dir[i][0];
            let nc = top.col+dir[i][1];
            let finstr = format!("{} {}",nr,nc);
            // println!("index {} row & col {} {} new row & col {}",i,top.row,top.col,finstr);
            if nr>-1 && nr<arr.len() as i32 && nc>-1 && nc<arr[0].len() as i32 && arr[nr as usize][nc as usize]!="." && !visited.contains(&finstr) || finstr==start{
                if top.count>1 && finstr==start{
                    let nodeno = (top.count+1)/2;
                    println!("nodeno {}",nodeno);
                    return;
                }
                let breaker = if ele=="S" {true} else {false};
                let combat_result = combat(i as i32,arr[nr as usize][nc as usize],pair.clone(),map.clone(),breaker);
                // println!("ele -> {} result - > {}",arr[nr as usize][nc as usize],combat_result);
                if combat_result==false {continue;}
                let node_data = Node{
                    row:nr,
                    col:nc,
                    count:top.count+1
                };
                // println!("node row {} col {} ele {} count {}",node_data.row,node_data.col,arr[node_data.row as usize][node_data.col as usize],node_data.count);
                all_nodes.push(node_data.clone());
                stack.push(node_data);
            }
        }
    }
}