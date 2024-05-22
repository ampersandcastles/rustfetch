use sys_info;
use colored::*;
use std::fs;

fn main() {
    let distro = detect_distro();
    let art = get_ascii_art(&distro);
    let sys_info = get_system_info();
    print_art_and_info_side_by_side(art, sys_info);
}

fn detect_distro() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("ID=") {
                return line.split('=').nth(1).unwrap_or("").to_string();
            }
        }
    }
    "unknown".to_string()
}

fn get_ascii_art(distro: &str) -> &'static str {
    match distro {
        "ubuntu" => r#"
          _
         | |
         | |===( )   //////  
         |_|   |||  | o o|
                ||| ( c  ) 
                 ||| \= / 
                  |||||||| 
                   ||||||  
    "#,
        "arch" => r#"
            /\\
           /  \\
          /\\   \\
         /      \\
        /   ,,   \\
       /   |  |  -\\
      /_-''    ''-_\\
    "#,
        "fedora" => r#"
         _______
       /        /\
      /        /  \
     /        /    \
    /________/______\
    \        \      /
     \        \    /
      \        \  /
       \________\/
    "#,
        _ => "",
    }
}

fn get_system_info() -> Vec<String> {
    let mut info = Vec::new();

    info.push(format!("{}@", whoami::username().green()));
    info.push(format!("{}", whoami::hostname().yellow()));

    match sys_info::os_type() {
        Ok(os_type) => info.push(format!("OS: {}", os_type)),
        Err(_) => info.push("OS: Unknown".to_string()),
    }

    match sys_info::os_release() {
        Ok(os_release) => info.push(format!("Kernel: {}", os_release)),
        Err(_) => info.push("Kernel: Unknown".to_string()),
    }

    match sys_info::cpu_speed() {
        Ok(cpu_speed) => info.push(format!("CPU Speed: {} MHz", cpu_speed)),
        Err(_) => info.push("CPU Speed: Unknown".to_string()),
    }

    match sys_info::mem_info() {
        Ok(mem) => {
            let used_memory = mem.total - mem.avail;
            info.push(format!("Memory: {} MB / {} MB", used_memory / 1024, mem.total / 1024));
        },
        Err(_) => info.push("Memory: Unknown".to_string()),
    }

    info
}

fn print_art_and_info_side_by_side(art: &str, sys_info: Vec<String>) {
    let art_lines: Vec<&str> = art.lines().collect();
    let info_lines = sys_info;

    let max_lines = art_lines.len().max(info_lines.len());

    for i in 0..max_lines {
        let art_line = if i < art_lines.len() {
            art_lines[i]
        } else {
            ""
        };
        let info_line = if i < info_lines.len() {
            &info_lines[i]
        } else {
            ""
        };
        println!("{:<30} {}", art_line, info_line);
    }
}
