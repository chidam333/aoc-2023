use clap::Parser;

#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf,
    red:i32,
    green:i32,
    blue:i32,
}
#[derive(Parser)]
struct Cli2{
    path:std::path::PathBuf
}
fn main(){
    let args = Cli2::parse();
    let content = std::fs::read_to_string(args.path).expect("File read error occured");
    let mut ans = 0;
    for line in content.lines(){
        let mut max = (0,0,0);
        let rstr = line.split(":").collect::<Vec<&str>>()[1];
        let ele = rstr.split(|n| n==',' || n==';').collect::<Vec<&str>>();
        for i in 0..ele.len(){
            let ele_str = ele[i].trim();
            let whitespace_index = ele_str.find(" ").expect("can't find whitespace");
            let (val,typ) = ele_str.split_at(whitespace_index);
            let val = val.parse::<i32>().expect("can't parse to int");
            let typ = typ.trim();
            if typ == "red"{max.0 = std::cmp::max(val, max.0)}
            if typ == "green"{max.1 = std::cmp::max(val,max.1)}
            if typ == "blue"{max.2 = std::cmp::max(val,max.2)}
        }
        ans += max.0*max.1*max.2;
    }
    println!("ans -  >  {}" ,ans);
}
fn first_main(){
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).expect("File read error occured");
    let mut ans = 0;
    for line in content.lines(){
        let rstr_arr = line.split(":").collect::<Vec<&str>>();
        let rstr_id = rstr_arr[0].split(" ").collect::<Vec<&str>>()[1];
        let rstr = rstr_arr[1];
        let ele_arr1 = rstr.split(';').collect::<Vec<&str>>();
        let mut ele_arr2:Vec<Vec<&str>> = vec![];
        for i in ele_arr1 {
            ele_arr2.push(i.split(",").collect::<Vec<&str>>());
        }
        let mut accepted_flag = true;
        for ele in ele_arr2{
            // ele.iter().for_each(|n| println!("ele - > {}",n));
            let mut start_cubes = (0,0,0);
            let mut index = 0;
            while index<ele.len() && start_cubes.0<=args.red && start_cubes.1<=args.green && start_cubes.2<=args.blue && accepted_flag{
                let ele_str = ele[index].trim();
                let whitespace_index = ele_str.find(" ").expect("can't find whitespace");
                let (val,typ) = ele_str.split_at(whitespace_index);
                // println!("{} {} {} {}",whitespace_index,typ,ele_str,val);
                let val = val.parse::<i32>().expect("can't parse to int");
                let typ = typ.trim();
                if typ == "red"{start_cubes.0+=val;}
                if typ == "green"{start_cubes.1+=val;}
                if typ == "blue"{start_cubes.2+=val;}
                index+=1;
            }
            if index<ele.len() || start_cubes.0>args.red || start_cubes.1>args.green || start_cubes.2>args.blue{
                accepted_flag = false;
            }
        }
        if accepted_flag {ans += rstr_id.parse::<i32>().expect("can't parse rstr_id");}
    }
    println!("ans - > {}",ans);
}