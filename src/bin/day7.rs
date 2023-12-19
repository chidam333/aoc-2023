use std::collections::HashMap;

use clap::Parser;
#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf
}
fn main(){
    fn map_insert<'a>(result:&mut HashMap<&'a str, Vec<&'a str>>,word: &'a str,str: &'a str){
        if result.contains_key(str){
            let mut cur_vec = result.get(str).unwrap().to_vec();
            cur_vec.push(word);
            result.insert(str, cur_vec);
        }else{
            let arr = vec![word];
            result.insert(str,arr);
        }
    }
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).unwrap();
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut input = HashMap::new();
    for line in content.lines(){
        let line_arr = line.split(" ").collect::<Vec<&str>>();
        let word = line_arr[0].trim();
        input.insert(word,line_arr[1]);
        let mut map = HashMap::new();
        let mut max_key = word.chars().next().unwrap();
        let mut max_use = 1;
        for i in word.chars(){
            if map.contains_key(&i){
                let cur_val = map.get(&i).unwrap();
                if max_use<cur_val+1 && i!='J'{
                    max_use=cur_val+1;
                    max_key=i;
                }
                map.insert(i, cur_val+1);
            }else{
                map.insert(i,1);
            }
        }
        if map.contains_key(&'J'){
            let tj = *map.get(&'J').unwrap();
            map.insert('J',0);
            map.insert(max_key, max_use+tj);
        }
        let mut has_three =0;
        let mut has_two =0;
        let mut has_four =0;
        let mut has_five = 0;
        for (k,v) in map{
            if v==2{
                has_two+=1;
            }
            if v==3{
                has_three+=1;
            }
            if v==4{
                has_four+=1;
            }
            if v==5{
                has_five+=1;
            }
        }
        if has_five==1{
            map_insert(&mut result, word, "five");
            continue;
        }if has_four==1{
            map_insert(&mut result, word, "four");
            continue;          
        }
        if has_two>0 && has_three>0{
            map_insert(&mut result, word, "full");
            continue;
        }
        if has_three==1{
            map_insert(&mut result, word, "three");
            continue;
        }
        if has_two == 2{
            map_insert(&mut result, word, "two");
            continue;
        }
        if has_two==1{
            map_insert(&mut result, word, "one");
            continue;
        }
        map_insert(&mut result, word, "high");
    }
    let mut rank = HashMap::new();
    let mut exiting_rank = 1;
    let mut ans = 0;
    let order = vec![
        ("A", 1), ("K", 2), ("Q", 3), ("T", 5), 
        ("9", 6), ("8", 7), ("7", 8), ("6", 9), ("5", 10),
        ("4", 11), ("3", 12), ("2", 13),("J",14)
    ].into_iter().collect::<HashMap<&str,usize>>();
    if result.contains_key("high"){
        let mut arr = result.get("high").unwrap().to_vec();
        arr.sort_by(|a,b| 
            {
                let mut achars = a.chars();
                let mut bchars = b.chars();
                let mut aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                let mut bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                while aval==bval{
                    aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                    bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                }
                return bval.cmp(&aval);
            }
        );
        for word in arr{
            rank.insert(word,exiting_rank);
            ans+= input.get(word).unwrap().parse::<i32>().unwrap() * exiting_rank;
            exiting_rank+=1;
        }
    }
    if result.contains_key("one"){
        let mut arr = result.get("one").unwrap().to_vec();
        arr.sort_by(|a,b| 
            {
                let mut achars = a.chars();
                let mut bchars = b.chars();
                let mut aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                let mut bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                while aval==bval{
                    aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                    bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                }
                return bval.cmp(&aval);
            }
        );
        for word in arr{
            rank.insert(word,exiting_rank);
            ans+= input.get(word).unwrap().parse::<i32>().unwrap() * exiting_rank;
            exiting_rank+=1;
        }
    }
    if result.contains_key("two"){
        let mut arr = result.get("two").unwrap().to_vec();

        arr.sort_by(|a,b| 
                {
                let mut achars = a.chars();
                let mut bchars = b.chars();
                let mut aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                let mut bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                while aval==bval{
                    aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                    bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                }
                return bval.cmp(&aval);
            }
        );
        for word in arr{
            rank.insert(word,exiting_rank);
            ans+= input.get(word).unwrap().parse::<i32>().unwrap() * exiting_rank;
            exiting_rank+=1;
        }
    }
    if result.contains_key("three"){
        let mut arr = result.get("three").unwrap().to_vec();
        arr.sort_by(|a,b| 
            {
                let mut achars = a.chars();
                let mut bchars = b.chars();
                let mut aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                let mut bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                while aval==bval{
                    aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                    bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                }
                return bval.cmp(&aval);
            }
        );
        for word in arr{
            rank.insert(word,exiting_rank);
            ans+= input.get(word).unwrap().parse::<i32>().unwrap() * exiting_rank;
            exiting_rank+=1;
        }
    }
    if result.contains_key("full"){
        let mut arr = result.get("full").unwrap().to_vec();
        arr.sort_by(|a,b| 
            {
                let mut achars = a.chars();
                let mut bchars = b.chars();
                let mut aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                let mut bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                while aval==bval{
                    aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                    bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                }
                return bval.cmp(&aval);
            }
        );
        for word in arr{
            rank.insert(word,exiting_rank);
            ans+= input.get(word).unwrap().parse::<i32>().unwrap() * exiting_rank;
            exiting_rank+=1;
        }
    }
    if result.contains_key("four"){
        let mut arr = result.get("four").unwrap().to_vec();
        arr.sort_by(|a,b| 
            {
                let mut achars = a.chars();
                let mut bchars = b.chars();
                let mut aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                let mut bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                while aval==bval{
                    aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                    bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                }
                return bval.cmp(&aval);
            }
        );
        for word in arr{
            rank.insert(word,exiting_rank);
            ans+= input.get(word).unwrap().parse::<i32>().unwrap() * exiting_rank;
            exiting_rank+=1;
        }
    }
    if result.contains_key("five"){
        let mut arr = result.get("five").unwrap().to_vec();
        arr.sort_by(|a,b| 
            {
                let mut achars = a.chars();
                let mut bchars = b.chars();
                let mut aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                let mut bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                while aval==bval{
                    aval = order.get(achars.next().unwrap().to_string().as_str()).unwrap();
                    bval = order.get(bchars.next().unwrap().to_string().as_str()).unwrap();
                }
                return bval.cmp(&aval);
            }
        );
        for word in arr{
            rank.insert(word,exiting_rank);
            ans+= input.get(word).unwrap().parse::<i32>().unwrap() * exiting_rank;
            exiting_rank+=1;
        }
    }
    println!("{} ",ans);
}