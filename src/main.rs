mod libs;
mod prompts;
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
        prompts::utils::clearstd();
        let chlist=[("下载MCreator",0),("配置Proxifier",1),("退出",2)];
        let worktype=Vec::from(chlist);
        let workstr= prompts::list::loopforres(worktype,"📦️ MCreator Tookit Rust".green().bold(),"(使用↑↓来选择选项，ENTER确认)".cyan().normal());
        match workstr {
            0=>{
                println!("Start Connact to Github API...");
                let vers = libs::handle::get_text("http://api.github.com/repos/MCreator/MCreator/releases");
                println!(" ✅ Success");
                break;
            },
            1=>{
                break;
            },
            2=>{
                break;
            }
            _=>println!("unhandle worktype")
        };
    }
}
