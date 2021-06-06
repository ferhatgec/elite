// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod parser;
pub mod read;
pub mod lexer;
pub mod tokenizer;
pub mod ast;
pub mod logger;

pub static VERSION: f32 = 0.1;

pub static VALID_VERSIONS: &'static [f32] = &[
    0.1
];
