use std::collections::HashMap;

use clap::Parser;

#[derive(Parser)]
struct Cli{
    path:std::path::PathBuf,
}
fn main(){
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).expect("problem in reading string");
    let mut lines = content.lines();
    let seeds = lines.next().expect("can't parse first line");
    let mut ssm = HashMap::new();
    while "seed-to-soil map:"!=lines.next().expect("can't parse ssm"){}
    let mut cur_line = lines.next().expect("can't parse 2");
    while ""!=cur_line{
        println!("line ssm {}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        let mut seed_no = line_arr[1].parse::<i32>().expect("can't parse str to int 1") as u128;
        let mut soil_no = line_arr[0].parse::<i32>().expect("can't parse str to int 1") as u128;
        let till = seed_no+line_arr[2].parse::<u128>().expect("can't parse str to int 1");
        while seed_no<till{
            ssm.insert(seed_no,soil_no);
            seed_no+=1;
            soil_no+=1
        }
        cur_line = lines.next().expect("can't parse");
    }
    lines.next().expect("can't parse 4");
    cur_line = lines.next().expect("can't parse 5");
    let mut sfm = HashMap::new();
    while ""!=cur_line{
        println!("line sfm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        let mut soil_no = line_arr[1].parse::<i32>().expect("can't parse str to int 2") as u128;
        let mut fert = line_arr[0].parse::<i32>().expect("can't parse str to int 2") as u128;
        let till = soil_no+line_arr[2].parse::<u128>().expect("can't parse str to int 2");
        while soil_no<till{
            sfm.insert(soil_no,fert);
            soil_no+=1;
            fert+=1
        }
        cur_line = lines.next().expect("can't parse");        
    }
    lines.next().expect("can't parse 6");
    cur_line = lines.next().expect("can't parse 7");
    let mut fwm = HashMap::new();
    while ""!=cur_line{
        println!("line fwm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        let mut fert = line_arr[1].parse::<i32>().expect("can't parse str to int 3") as u128;
        let mut wat = line_arr[0].parse::<i32>().expect("can't parse str to int 3") as u128;
        let till = fert+line_arr[2].parse::<u128>().expect("can't parse str to int 3");
        while fert<till{
            fwm.insert(fert,wat);
            fert+=1;
            wat+=1
        }
        cur_line = lines.next().expect("can't parse");        
    }
    lines.next().expect("can't parse 8");
    cur_line = lines.next().expect("can't parse 9");
    let mut wlm = HashMap::new();
    while ""!=cur_line{
        println!("line wlm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        let mut wat = line_arr[1].parse::<i32>().expect("can't parse str to int 4") as u128;
        let mut light = line_arr[0].parse::<i32>().expect("can't parse str to int 4") as u128;
        let till = wat+line_arr[2].parse::<u128>().expect("can't parse str to int 4");
        while wat<till{
            wlm.insert(wat,light);
            wat+=1;
            light+=1
        }
        cur_line = lines.next().expect("can't parse");        

    }
    lines.next().expect("can't parse 9*");
    cur_line = lines.next().expect("can't parse 10");
    let mut ltm = HashMap::new();
    while ""!=cur_line{
        println!("line ltm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        let mut ligh = line_arr[1].parse::<i32>().expect("can't parse str to int 5") as u128;
        let mut temp = line_arr[0].parse::<i32>().expect("can't parse str to int 5") as u128;
        let till = ligh+line_arr[2].parse::<u128>().expect("can't parse str to int 5");
        while ligh<till{
            ltm.insert(ligh,temp);
            ligh+=1;
            temp+=1
        }
        cur_line = lines.next().expect("can't parse");        

    }
    lines.next().expect("can't parse 11");
    cur_line = lines.next().expect("can't parse 12");
    let mut thm = HashMap::new();
    while ""!=cur_line{
        println!("line thm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        let mut temp = line_arr[1].parse::<i32>().expect("can't parse str to int 6") as u128;
        let mut hum = line_arr[0].parse::<i32>().expect("can't parse str to int 6") as u128;
        let till = temp+line_arr[2].parse::<u128>().expect("can't parse str to int 6");
        while temp<till{
            thm.insert(temp,hum);
            temp+=1;
            hum+=1
        }
        cur_line = lines.next().expect("can't parse");        

    }
    lines.next().expect("can't parse 13");
    cur_line = lines.next().expect("can't parse 14");
    let mut hlm = HashMap::new();
    while ""!=cur_line{
        println!("line hlm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        let mut hum = line_arr[1].parse::<i32>().expect("can't parse str to int 7") as u128;
        let mut loc = line_arr[0].parse::<i32>().expect("can't parse str to int 7") as u128;
        let till = hum+line_arr[2].parse::<u128>().expect("can't parse str to int 7");
        while hum<till{
            hlm.insert(hum,loc);
            hum+=1;
            loc+=1
        }
        cur_line = lines.next().expect("can't parse");        

    }

    let seeds = seeds.split(":").collect::<Vec<&str>>()[1];
    let seeds = seeds.trim().split(" ").map(|s| s.parse::<i32>().expect("seed no parse err")).collect::<Vec<i32>>();
    let mut min = std::u128::MAX;
    for i in seeds{
        let i = i as u128;
        let soil_no;
        let fert;
        let wat;
        let light;
        let temp;
        let hum;
        let loc;
        if ssm.contains_key(&i){
            soil_no = ssm.get(&i).expect("can't get soil number");
        }else{soil_no = &i;}
        if sfm.contains_key(&soil_no){
            fert = sfm.get(&soil_no).expect("can't get fert number");
        }else{fert = &soil_no;}
        if fwm.contains_key(&fert){
            wat = fwm.get(&fert).expect("can't get wat number");
        }else{wat = &fert;}
        if wlm.contains_key(&wat){
            light = wlm.get(&wat).expect("can't get light number");
        }else{light = &fert;}
        if ltm.contains_key(&light){
            temp = ltm.get(&light).expect("can't get temp number");
        }else{temp = &light;}
        if thm.contains_key(&temp){
            hum = thm.get(&temp).expect("can't get hum number");
        }else{hum = &temp;}
        if hlm.contains_key(&hum){
            loc = hlm.get(&hum).expect("can't get hum number");
        }else{loc = &hum;}
        println!("soil {} fert {} water {} light {} temp {} hum {} loc {}",soil_no,fert,wat,light,temp,hum,loc);
        min = std::cmp::min(min, *loc);
    }
    println!("min {}",min);
}