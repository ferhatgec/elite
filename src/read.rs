// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use {
    std::io::BufRead,

    crate::{
        logger::{
            EliteLogType,
            elite_logger
        }
    }
};

pub struct EliteFileData {
    pub raw_data: String,
    pub unparsed: Vec<String>
}

pub mod elite_file_contents {
    // to_lowercase(filename) supported.
    // filename: EliTeFilE, eliteFILE, EliteFile etc.
    pub const GENERIC_FILENAME: &str = "elitefile";

    pub fn create_empty_string() -> String {
        return "".to_string();
    }
}

impl EliteFileData {
    pub fn check_is_elite(&self, file: &String) -> bool {
        return if file.to_ascii_lowercase() == elite_file_contents::GENERIC_FILENAME {
            true
        } else {
            false
        };
    }

    pub fn read_raw_file(&mut self, file: &String) {
        if !self.check_is_elite(file) {
            elite_logger::log(EliteLogType::Info,
                              "filename",
                              &format!("to_lowercase({}) ignored", file));
        }

        let mut raw_data = String::new();

        if let Ok(lines) = self.read_lines(file) {
            for line in lines {
                if let Ok(ip) = line {
                    if crate::tokenizer::elite_tokenizer::is_comment(&ip.as_str()) {
                        continue;
                    }

                    raw_data.push(' '); raw_data.push_str(&ip); raw_data.push('\n');
                }
            }
        }

        self.raw_data = raw_data;
    }

    fn read_lines<P>(&self, file: &P) -> std::io::Result<
        std::io::Lines<std::io::BufReader<std::fs::File>>
    > where P: AsRef<std::path::Path>, {
        Ok(std::io::BufReader::new(
            std::fs::File::open(file)?).lines())
    }
}