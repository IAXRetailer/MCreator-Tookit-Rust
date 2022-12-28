use std::{ops::Add};

use colored::{Colorize, ColoredString};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use super::utils::clearstd;


pub fn loopforres(choices:Vec<(&str,i32)>,ques:ColoredString,tips:ColoredString) -> i32{
    clearstd();
    let itor=choices.iter();
    let mut vector: Vec<&str> = Vec::new();
    for (k,_) in itor{ vector.push(k); }
    println!("{} {}",ques,tips);
    for i in renderchoice(vector, 0).iter(){
        println!("{}",i)
    }
    let maxnowco=choices.len()-1;
    let mut nowco=0;
    loop {
        let mut vector2: Vec<&str> = Vec::new();
        let itor=choices.iter().copied();
        for (k,_) in itor{ vector2.push(k); }
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: _,
                kind:_,
                state:_,
            }) => {
                if nowco == maxnowco{
                    nowco = 0;
                    modifykey(vector2,ques.clone(),tips.clone(),0);
                }else{
                    nowco=nowco.add(1);
                    modifykey(vector2,ques.clone(),tips.clone(),nowco.try_into().unwrap());
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: _,
                kind:_,
                state:_,
            }) => {
                if nowco == 0{
                    nowco = maxnowco;
                    modifykey(vector2,ques.clone(),tips.clone(),maxnowco.try_into().unwrap());
                }else{
                    nowco=nowco - 1;
                    modifykey(vector2,ques.clone(),tips.clone(),nowco.try_into().unwrap());
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: _,
                kind:_,
                state:_,
            }) => {
                let data=vector2.get(nowco as usize).unwrap() as &str;
                clearstd();
                println!("{} {}",ques,data.yellow());
                for (k,v) in choices.iter().copied(){
                    if data == k{
                        return v;
                    }
                }
            },
            _ => (),
        }
    }
}

fn modifykey(vec:Vec<&str>,ques:ColoredString,tips:ColoredString,nowco:i32){
    clearstd();
    println!("{} {}",ques,tips);
    for i in renderchoice(vec, nowco).iter(){
        println!("{}",i)
    }
}


fn renderchoice(vec:Vec<&str>,nowco:i32) -> Vec<ColoredString>{
    let mut finvec:Vec<ColoredString> = Vec::new();
    for i in vec.iter(){
        if vec.get(nowco as usize).unwrap() == i{
            finvec.push(format!(" ❯  {}",i).bright_green());
        }else {
            finvec.push(format!("    {}",i).normal());
        }
    }
    return finvec;
}