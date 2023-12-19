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
    let mut ssm = vec![];
    while "seed-to-soil map:"!=lines.next().expect("can't parse ssm"){}
    let mut cur_line = lines.next().expect("can't parse 2");
    while ""!=cur_line{
        println!("line ssm {}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        ssm.push(line_arr);
        cur_line = lines.next().expect("can't parse");
    }
    lines.next().expect("can't parse 4");
    cur_line = lines.next().expect("can't parse 5");
    let mut sfm = vec![];
    while ""!=cur_line{
        println!("line sfm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        sfm.push(line_arr);
        cur_line = lines.next().expect("can't parse");        
    }
    lines.next().expect("can't parse 6");
    cur_line = lines.next().expect("can't parse 7");
    let mut fwm = vec![];
    while ""!=cur_line{
        println!("line fwm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        fwm.push(line_arr);
        cur_line = lines.next().expect("can't parse");        
    }
    lines.next().expect("can't parse 8");
    cur_line = lines.next().expect("can't parse 9");
    let mut wlm = vec![];
    while ""!=cur_line{
        println!("line wlm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        wlm.push(line_arr);
        cur_line = lines.next().expect("can't parse");        

    }
    lines.next().expect("can't parse 9*");
    cur_line = lines.next().expect("can't parse 10");
    let mut ltm = vec![];
    while ""!=cur_line{
        println!("line ltm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        ltm.push(line_arr);
        cur_line = lines.next().expect("can't parse");        

    }
    lines.next().expect("can't parse 11");
    cur_line = lines.next().expect("can't parse 12");
    let mut thm = vec![];
    while ""!=cur_line{
        println!("line thm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        thm.push(line_arr);
        cur_line = lines.next().expect("can't parse");        

    }
    lines.next().expect("can't parse 13");
    cur_line = lines.next().expect("can't parse 14");
    let mut hlm = vec![];
    while ""!=cur_line{
        println!("line hlm{}",cur_line);
        let line_arr = cur_line.trim().split(" ").collect::<Vec<&str>>();
        hlm.push(line_arr);
        cur_line = lines.next().expect("can't parse");        

    }

    let seeds = seeds.split(":").collect::<Vec<&str>>()[1];
    let seeds = seeds.trim().split(" ").map(|s| s.parse::<i128>().expect("seed no parse err")).collect::<Vec<i128>>();
    let mut min = std::i128::MAX;
    for seed in seeds{
        let mut seed = seed as i128;
        let mut soil_no = None;
        let mut fert = None;
        let mut wat = None;
        let mut light = None;
        let mut temp = None;
        let mut hum = None;
        let mut loc = None;
        for i in ssm.clone(){
            let start = i[1].parse::<i128>().expect("start parse error") as i128;
            let end = start + i[2].parse::<i128>().expect("start parse error") as i128;
            let val = i[0].parse::<i128>().expect("val parse error") as i128;
            if seed<end && seed>start{
                soil_no = Some(val+(seed-start));
                // println!("soil_no {:?} val {} seed {} start {}",soil_no,val,seed,start);
            }
        }
        if soil_no==None{soil_no = Some(seed);}
        for i in sfm.clone(){
            let start = i[1].parse::<i128>().expect("start parse error") as i128;
            let end = start + i[2].parse::<i128>().expect("start parse error") as i128;
            let val = i[0].parse::<i128>().expect("val parse error") as i128;
            let soil_no = soil_no.unwrap();
            if soil_no<end && soil_no>=start{
                fert = Some(val+(soil_no-start).abs());
                // println!("fert {:?} soil_no {} start {} val {}",fert,soil_no,start,val);
            }
        }
        if fert==None{fert = soil_no;}
        for i in fwm.clone(){
            let start = i[1].parse::<i128>().expect("start parse error") as i128;
            let end = start + i[2].parse::<i128>().expect("start parse error") as i128;
            let val = i[0].parse::<i128>().expect("val parse error") as i128;
            let fert = fert.unwrap();
            if fert<end && fert>=start{
                wat = Some(val+(fert-start).abs());
                // println!("wat {:?} fert {} start {} val {}",wat,fert,start,val);
            }          
        }
        if wat==None{wat = fert;}
        for i in wlm.clone(){
            let start = i[1].parse::<i128>().expect("start parse error") as i128;
            let end = start + i[2].parse::<i128>().expect("start parse error") as i128;
            let val = i[0].parse::<i128>().expect("val parse error") as i128;
            let wat: i128 = wat.unwrap();
            if wat<end && wat>=start{
                light = Some(val+(wat-start).abs());
            }          
        }
        if light==None{light = wat;}
        for i in ltm.clone(){
            let start = i[1].parse::<i128>().expect("start parse error") as i128;
            let end = start + i[2].parse::<i128>().expect("start parse error") as i128;
            let val = i[0].parse::<i128>().expect("val parse error") as i128;
            let light = light.unwrap();
            if light<end && light>=start{
                temp = Some(val+(light-start).abs());
            }          
        }
        if temp==None{temp = light;}
        for i in thm.clone(){
            let start = i[1].parse::<i128>().expect("start parse error") as i128;
            let end = start + i[2].parse::<i128>().expect("start parse error") as i128;
            let val = i[0].parse::<i128>().expect("val parse error") as i128;
            let temp = temp.unwrap();
            if temp<end && temp>=start{
                hum = Some(val+(temp-start).abs());
            }                 
        }if hum==None{hum=temp;}
        for i in hlm.clone(){
            let start = i[1].parse::<i128>().expect("start parse error") as i128;
            let end = start + i[2].parse::<i128>().expect("start parse error") as i128;
            let val = i[0].parse::<i128>().expect("val parse error") as i128;
            let hum = hum.unwrap();
            if hum<end && hum>=start{
                loc = Some(val+(hum-start));
                //println!("loc {} fert {} start {} val {}",loc.unwrap(),hum,start,val);
            }                 
        }if loc==None{loc=hum;}
        println!("seed {} soil {} fert {} water {} light {} temp {} hum {} loc {}",seed,soil_no.unwrap(),fert.unwrap(),wat.unwrap(),light.unwrap(),temp.unwrap(),hum.unwrap(),loc.unwrap());
        min = std::cmp::min(min, loc.unwrap());
    }
    println!("min {}",min);
}