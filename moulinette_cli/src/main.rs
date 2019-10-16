extern crate clap;
extern crate moulinette;
extern crate shiplift;


use clap::{App, SubCommand};

mod docker;

fn main() {
    let matches = App::new("Moulinette cli")
        .version("1.0")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("Run moulinette pipeplines")
        .subcommand(
            SubCommand::with_name("run")
                .about("run pipeline")
                .version("1.0"),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("run") {
        // let config = moulinette::get_config();
        // println!("{:?}", config);
        // docker::pull_image();
        docker::run_container();
    }
}
