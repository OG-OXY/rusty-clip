use arboard::Clipboard;
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rusty-clip", about = "Lightning-fast Wayland clipboard daemon & sanitizer")]
struct Args {
    /// Keep trailing newlines instead of stripping them
    #[arg(long)]
    keep_newline: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run as the persistent background clipboard daemon
    Daemon,
}

fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(runtime_dir).join("rusty-clip.sock")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Daemon) => run_daemon(),
        None => run_client(args.keep_newline),
    }
}

fn run_client(keep_newline: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let cleaned = if keep_newline {
        input.trim_end_matches(|c: char| c == '\r')
    } else {
        input.trim_end_matches(|c: char| c == '\n' || c == '\r' || c.is_whitespace())
    };

    // Connect to the background daemon's Unix socket and stream the cleaned text
    let path = socket_path();
    let mut stream = UnixStream::connect(&path).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::NotConnected,
            "rusty-clip daemon is not running. Is the systemd service active?",
        )
    })?;
    
    stream.write_all(cleaned.as_bytes())?;

    Ok(())
}

fn run_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let path = socket_path();
    if path.exists() {
        fs::remove_file(&path)?;
    }

    let listener = UnixListener::bind(&path)?;
    let mut clipboard = Clipboard::new()?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut text = String::new();
                if stream.read_to_string(&mut text).is_ok() {
                    if let Err(e) = clipboard.set().text(text) {
                        eprintln!("Failed to update clipboard: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Socket connection error: {}", e);
            }
        }
    }

    Ok(())
}
