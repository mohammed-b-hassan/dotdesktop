use clap::Parser;
use std::{fs::File, io::Write};

const DEFAULT_NAME: &str = "app";
const DEFAULT_EXECUTABLE: &str = "";
const DEFAULT_TYPE: &str = "Application";
const DEFAULT_MIME_TYPE: &str = "";
const DEFAULT_ICON: &str = "";
const DEFAULT_PATH: &str = "";
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long, default_value_t=DEFAULT_NAME.to_string())]
    name: String,
    #[arg(short = 'e', long, default_value_t=DEFAULT_EXECUTABLE.to_string())]
    exec: String,
    #[arg(short = 't', long, default_value_t=DEFAULT_TYPE.to_string())]
    bin_type: String,
    #[arg(short = 'm', long, default_value_t=DEFAULT_MIME_TYPE.to_string())]
    mime_type: String,
    #[arg(short = 'd', long, action = clap::ArgAction::SetTrue)]
    no_display: bool,
    #[arg(short = 's', long, action = clap::ArgAction::SetTrue)]
    startup_notify: bool,
    #[arg(short = 'i', long, default_value_t=DEFAULT_ICON.to_string())]
    icon: String,
    #[arg(short = 'p', long, default_value_t=DEFAULT_PATH.to_string())]
    path: String,
    #[arg(short = 'y', long, action = clap::ArgAction::SetTrue)]
    yes_all: bool,
    #[arg(short = 'T', long, action = clap::ArgAction::SetTrue)]
    terminal: bool,
}

fn main() {
    let args = Args::parse();
    let path = if args.path.len() <= 0 {
        dirs::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            + "/.local/share/applications/"
    } else {
        String::from(&args.path)
    };
    let file_path = format!("{}{}.desktop", path, args.name.replace(" ", "-"));

    if !args.yes_all {
        let mut file = File::create(&file_path).unwrap();
        let content = format!(
            "[Desktop Entry]\nType={}\nName={}\nMimeType={}\nExec={}\nNoDisplay={}\nStartupNotify={}\nIcon={}\nTerminal={}",
            args.bin_type,
            args.name,
            args.mime_type,
            args.exec,
            args.no_display,
            args.startup_notify,
            args.icon,
            args.terminal
        );

        let _ = file.write_all(content.as_bytes()).unwrap();
    } else {
        let mut file = File::create(&file_path).unwrap();
        let content = format!(
            "[Desktop Entry]\nType={}\nName={}\nMimeType={}\nExec={}\nNoDisplay={}\nStartupNotify={}\nIcon={}\nTerminal={}",
            args.bin_type,
            args.name,
            args.mime_type,
            args.exec,
            args.no_display,
            args.startup_notify,
            args.icon,
            args.terminal
        );

        let _ = file.write_all(content.as_bytes()).unwrap();
    }

    println!("Name: {}", args.name);
    println!("Executable: {}", args.exec);
    println!("Type: {}", args.bin_type);
    println!("Mime type: {}", args.mime_type);
    println!("Path: {}", path);
    println!("Icon: {}", args.icon);
    println!("No display: {}", args.no_display);
    println!("Startup notify: {}", args.startup_notify);
    println!("File saved to: {}", file_path)
}
