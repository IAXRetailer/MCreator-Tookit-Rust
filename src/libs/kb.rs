use std::{collections::LinkedList, ops::Add};

use colored::{Colorize, ColoredString};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crate::libs::utils::clearstd;


pub fn loopforres(ques:LinkedList<&str>,tips:ColoredString)-> &str{
    let itor=ques.iter();
    let mut vector: Vec<&str> = Vec::new();
    for i in itor{ vector.push(i); }
    println!("{}",tips);
    for i in renderchoice(vector, 0).iter(){
        println!("{}",i)
    }
    let maxnowco=ques.len()-1;
    let mut nowco=0;
    loop {
        let mut vector2: Vec<&str> = Vec::new();
        let itor=ques.iter();
        for i in itor{ vector2.push(i); }
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: _,
                kind:_,
                state:_,
            }) => {
                if nowco == maxnowco{
                    nowco = 0;
                    modifykey(vector2,tips.clone(),0);
                }else{
                    nowco=nowco.add(1);
                    modifykey(vector2,tips.clone(),nowco.try_into().unwrap());
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
                    modifykey(vector2,tips.clone(),maxnowco.try_into().unwrap());
                }else{
                    nowco=nowco - 1;
                    modifykey(vector2,tips.clone(),nowco.try_into().unwrap());
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: _,
                kind:_,
                state:_,
            }) => {
                let data=vector2.get(nowco as usize);
                return data.unwrap();
            },
            _ => (),
        }
    }
}

fn modifykey(vec:Vec<&str>,tips:ColoredString,nowco:i32){
    clearstd();
    println!("{}",tips);
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