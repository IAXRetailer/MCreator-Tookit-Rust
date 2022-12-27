use std::{collections::LinkedList};

use colored::{Colorize, ColoredString};
//use colored::*;

pub fn loopforres(ques:LinkedList<&str>){
    loop {
        let itor=ques.iter();
        let mut vector: Vec<&str> = Vec::new();
        let nowco = 0;
        for i in itor{ vector.push(i); }
        for i in renderchoice(vector, nowco).iter(){
            println!(" ❯  {}",i)
        }
        break;
    }
}

fn renderchoice(vec:Vec<&str>,nowco:i32) -> Vec<ColoredString>{
    let mut finvec:Vec<ColoredString> = Vec::new();
    for i in vec.iter(){
        if vec.get(nowco as usize).unwrap() == i{
            finvec.push(i.bright_green());
        }else {
            finvec.push(i.normal());
        }
    }
    return finvec;
}