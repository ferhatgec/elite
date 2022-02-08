// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//
extern crate elite;

use elite::*;

const __VERSION__: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let cli_arguments: Vec<_> = std::env::args().collect();

    if cli_arguments.len() < 2 {
        println!("Fegeya Elite - small, powerful build system (version: {})\n\
                -------\n\
                Usage:\n \
                {arg} file argument\n \
                {arg} Elitefile install", __VERSION__.unwrap_or("undefined"),
                                          arg = cli_arguments[0]);

        std::process::exit(1);
    }

    let mut elite_read = crate::read::EliteFileData {
        raw_data: crate::read::elite_file_contents::create_empty_string(),
        unparsed: vec![]
    };

    elite_read.read_raw_file(cli_arguments.get(1).unwrap());
    crate::lexer::elite_lexer::init_lexer(&elite_read, false);
}
