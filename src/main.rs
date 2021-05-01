// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

mod parser;
mod read;
mod lexer;
mod tokenizer;
mod ast;

fn main() {
    let cli_arguments: Vec<_> = std::env::args().collect();

    if cli_arguments.len() < 2 {
        println!("Fegeya Elite - small, powerful build system");

        std::process::exit(1);
    }

    let mut elite_read = crate::read::EliteFileData {
        raw_data: crate::read::elite_file_contents::create_empty_string(),
        unparsed: vec![]
    };

    elite_read.read_raw_file(cli_arguments.get(1).unwrap());
    crate::lexer::elite_lexer::init_lexer(&elite_read);
}
