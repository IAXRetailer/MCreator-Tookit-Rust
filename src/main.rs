mod libs;

use colored::Colorize;
fn main() {
    #[cfg(windows)]
    if cfg!(target_os="windows"){
        use colored::control;
        match control::set_virtual_terminal(true){
            Ok(_)=>"",
            Err(_)=>""
        };
    }
    loop {
        libs::utils::clearstd();
        let chlist=["下载MCreator","配置Proxifier","退出"];
        let worktype=Vec::from(chlist);
        let workstr= libs::kb::loopforres(worktype,format!("{} ({})","📦️ MCreator Tookit Rust".green().bold(),"使用↑↓来选择选项，ENTER确认".cyan()).normal());
        libs::utils::clearstd();
        println!("🔧 Chose work as {}",workstr.green().bold());
        match workstr {
            "退出"=>break,
            "下载MCreator"=>{
                break;
            },
            "配置Proxifier"=>{
                break;
            }
            _=>""
        };
    }
}
