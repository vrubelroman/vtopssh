mod app;
mod collector;
mod config;
mod model;
mod theme;
mod ui;

use anyhow::Result;
use config::AppConfig;
use std::{env, process};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    match parse_cli_args(env::args().skip(1)) {
        Ok(CliAction::Run) => {}
        Ok(CliAction::PrintHelp) => {
            print_help();
            return Ok(());
        }
        Ok(CliAction::PrintVersion) => {
            println!("{APP_NAME} {APP_VERSION}");
            return Ok(());
        }
        Err(message) => {
            eprintln!("{message}");
            eprintln!("Try '{APP_NAME} --help' for usage.");
            process::exit(2);
        }
    }

    let config = AppConfig::load()?;
    app::run(config)
}

#[derive(Debug, Eq, PartialEq)]
enum CliAction {
    Run,
    PrintHelp,
    PrintVersion,
}

fn parse_cli_args<I>(args: I) -> std::result::Result<CliAction, String>
where
    I: IntoIterator<Item = String>,
{
    let mut action = CliAction::Run;

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => action = CliAction::PrintHelp,
            "-V" | "--version" => action = CliAction::PrintVersion,
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }

    Ok(action)
}

fn print_help() {
    println!(
        "\
{APP_NAME} {APP_VERSION}
Terminal UI system monitor for Linux with remote host support over SSH

Usage:
  {APP_NAME} [OPTIONS]

Options:
  -h, --help       Print this help message
  -V, --version    Print version information"
    );
}

#[cfg(test)]
mod tests {
    use super::{parse_cli_args, CliAction};

    #[test]
    fn accepts_help_flag() {
        let action = parse_cli_args(["--help".to_string()]).unwrap();
        assert_eq!(action, CliAction::PrintHelp);
    }

    #[test]
    fn accepts_version_flag() {
        let action = parse_cli_args(["-V".to_string()]).unwrap();
        assert_eq!(action, CliAction::PrintVersion);
    }

    #[test]
    fn rejects_unknown_flag() {
        let error = parse_cli_args(["--wat".to_string()]).unwrap_err();
        assert!(error.contains("--wat"));
    }
}
