mod libs;
use std::{io::stdout, collections::LinkedList};

fn main() {
    loop {
        let std = stdout();
        match libs::utils::buildmainstd(std){
            Ok(_) => {}
            Err(e) => { println!("caught err {} in buildmainstd",e) }
        }
        let worktype=LinkedList::from(["下载MCreator","配置Proxifier","退出"]);
        libs::kb::loopforres(worktype);
        break;
    }
}
