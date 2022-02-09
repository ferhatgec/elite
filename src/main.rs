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
                {arg} ast Elitefile", __VERSION__.unwrap_or("undefined"),
                                          arg = cli_arguments[0]);

        std::process::exit(1);
    }

    if cli_arguments.get(1).unwrap() == "ast" && cli_arguments.len() < 3 {
        println!("{} ast Elitefile\n{}\n{}", cli_arguments[0],
                 "          ^^^^^^^^^",
                 "          file required");

        std::process::exit(1);
    }

    let mut elite_read = crate::read::EliteFileData {
        raw_data: crate::read::elite_file_contents::create_empty_string(),
        unparsed: vec![]
    };

    if cli_arguments.get(1).unwrap() != "ast" {
        elite_read.read_raw_file(cli_arguments.get(1).unwrap());
    } else { elite_read.read_raw_file(cli_arguments.get(2).unwrap()); }

    let mut x = crate::lexer::elite_lexer::init_lexer(&elite_read, if cli_arguments.get(1).unwrap() == "ast" {
        true
    } else { false });

    if cli_arguments.get(1).unwrap() == "ast" {
        x.ast_nodes.search(Branch::Data);
    }
}
