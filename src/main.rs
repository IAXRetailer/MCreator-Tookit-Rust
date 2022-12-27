mod libs;
use std::collections::LinkedList;

use colored::Colorize;


fn main() {
    loop {
        libs::utils::clearstd();
        let worktype=LinkedList::from(["下载MCreator","配置Proxifier","退出"]);
        let workstr= libs::kb::loopforres(worktype,format!("{} ({})","📦️ MCreator Tookit Rust".green().bold(),"使用↑↓来选择选项，ENTER确认".cyan()).normal());
        libs::utils::clearstd();
        println!("🔧 Chose work as {}",workstr.green().bold());
        match workstr {
            "退出"=>break,
            _=>""
        };
    }
}
