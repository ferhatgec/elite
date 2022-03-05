// MIT License
//
// Copyright (c) 2021-2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//
extern crate elite;

use elite::*;
use elite::ast::Branch;

const __VERSION__: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let cli_arguments: Vec<_> = std::env::args().collect();

    if cli_arguments.len() < 2 {
        println!("Fegeya Elite - small, powerful build system (version: {})\n\
                -------\n\
                Usage:\n \
                {arg} file argument\n \
                {arg} Elitefile install\n \
                {arg} analyze Elitefile\n \
                {arg} ast Elitefile", __VERSION__.unwrap_or("undefined"),
                                          arg = cli_arguments[0]);

        std::process::exit(1);
    }

    let arg = cli_arguments.get(1).unwrap();
    if (arg == "ast" || arg == "analyze") && cli_arguments.len() < 3 {
        println!("{} {} Elitefile\n{}^^^^^^^^^\n{}", cli_arguments[0],
                 &arg,
                 " ".repeat(cli_arguments[0].len() + arg.len() + 2),
                 "          file required");

        std::process::exit(1);
    }

    let mut elite_read = crate::read::EliteFileData {
        raw_data: crate::read::elite_file_contents::create_empty_string(),
        unparsed: vec![]
    };

    let val = match cli_arguments.get(1).unwrap().as_str() {
        "ast" |
        "analyze" => {
            elite_read.read_raw_file(cli_arguments.get(2).unwrap());
            true
        },
        _ => {
            elite_read.read_raw_file(cli_arguments.get(1).unwrap());
            false
        }
    };

    let mut x = crate::lexer::elite_lexer::init_lexer(&elite_read, val);

    match cli_arguments.get(1).unwrap().as_str() {
        "ast" =>  x.ast_nodes.search(Branch::Data),
        "analyze" => {
            println!("Possible arguments:");

            if x.arguments.is_empty() {
                println!("[not defined]");
            } else {
                for arg in &x.arguments {
                    println!(" * {}", arg);
                }
            }

            println!("\nPossible platforms:");

            if x.platforms.is_empty() {
                println!("[not defined]");
            } else {
                for platform in &x.platforms {
                    println!(" * {}", platform);
                }
            }
        },
        _ => {}
    }
}
