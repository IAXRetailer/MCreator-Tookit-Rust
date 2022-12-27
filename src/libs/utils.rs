use std::io::{Stdout};
use colored::*;
use crossterm::{terminal, ExecutableCommand,Result};

pub fn buildmainstd(mut std:Stdout) -> Result<()>{
    std.execute(terminal::Clear(terminal::ClearType::All))?;
    println!("{}","📦️ MCreator Tookit Rust".green().bold());
    Ok(())
}