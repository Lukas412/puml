use std::process;

use clap::{Arg, ArgAction, Command};
use eyre::Context;

use self::url::PumlUrlCreator;

mod url;

fn main() -> eyre::Result<()> {
    let cli = Command::new("puml")
        .subcommands([
            Command::new("run").about("run the docker plantuml server"),
            Command::new("open")
                .args([
                    Arg::new("file")
                        .required(true)
                        .num_args(1)
                        .help("the file to open"),
                    Arg::new("pdf")
                        .long("pdf")
                        .action(ArgAction::SetTrue)
                        .help("open as pdf"),
                    Arg::new("png")
                        .long("png")
                        .action(ArgAction::SetTrue)
                        .help("open as png"),
                    Arg::new("ascii")
                        .long("ascii")
                        .action(ArgAction::SetTrue)
                        .help("open as ascii"),
                ])
                .about("open a file in the browser"),
        ])
        .subcommand_required(true)
        .about("a plantuml cli providing a few extra scripts")
        .get_matches();

    match cli.subcommand().unwrap() {
        ("run", _) => {
            let command = process::Command::new("docker")
                .args([
                    "run",
                    "-d",
                    "-p",
                    "8080:8080",
                    "plantuml/plantuml-server:jetty",
                ])
                .status()?;
            println!("{}", command);
        }
        ("open", command) => {
            let file: &String = command.get_one("file").unwrap();

            let url_creator = PumlUrlCreator::new()?;

            let url = if command.get_flag("png") {
                url_creator.create_png_url(file)
            } else if command.get_flag("pdf") {
                url_creator.create_pdf_url(file)
            } else if command.get_flag("ascii") {
                url_creator.create_ascii_url(file)
            } else {
                url_creator.create_svg_url(file)
            }?;

            _ = process::Command::new("ff")
                .args(["-new-tab", &url])
                .status()
                .wrap_err("could not execute firefox command")?;

            println!("{}", url);
        }
        _ => {}
    };

    Ok(())
}
