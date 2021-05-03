// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

#[derive(PartialEq)]
pub enum EliteLogType {
    Success,
    Warning,
    Info   ,
    Error
}

pub mod elite_logger {
    use crate::logger::EliteLogType;

    pub fn log(_type: EliteLogType, token: &str, comment: &str) {
        println!("[{}] (\x1b[0;96m{}\x1b[0m) : {}", match _type {
                EliteLogType::Success => {
                    format!("{}Success{}", "\x1b[1;93m", "\x1b[0m")
                },
                EliteLogType::Warning => {
                    format!("{}Warning{}", "\x1b[0;91m", "\x1b[0m")
                },
                EliteLogType::Info    => {
                    format!("{}Info   {}"   , "\x1b[0;94m", "\x1b[0m")
                },
                EliteLogType::Error   => {
                    format!("{}Error  {}"  , "\x1b[1;31m", "\x1b[0m")
                }
            }, token, comment);

        if _type == EliteLogType::Error {
            std::process::exit(1);
        }
    }
}