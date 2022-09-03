// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_tokenizer {
    use crate::logger::{ EliteLogType, elite_logger };

    static TOKEN_LIST: &'static [char] = &[
        '(',
        ')',
        '[',
        ']',
        '$',
        ','
    ];

    pub fn tokenize_first(raw_data: &crate::read::EliteFileData) -> Vec<String> {
        let mut tokenized_data: Vec<String> = vec![];
        let mut current_token = String::new();
        let mut escape_sequence = false;
        let mut data = false;
        let mut skip_until_newline = false;
        let mut is_env     = false;
        let mut is_link    = false;
        let mut is_std     = false;
        let mut is_outfile = false;

        for ch in raw_data.raw_data.chars() {
            if ch != '\n' && skip_until_newline {
                continue;
            }

            match ch {
                '\n' => {
                    if skip_until_newline {
                        skip_until_newline = false;
                    } else if data {
                        current_token.push_str("\\n");
                    }
                },
                '(' |
                ')' |
                '[' |
                ']' |
                '$' |
                ',' |
                ' ' => {
                    if data {
                        current_token.push(ch);
                    } else {
                        if is_preprocessor_token(&current_token, "env") {
                            is_env = true;
                        } else if is_preprocessor_token(&current_token, "link") {
                            is_link = true;
                        } else if is_preprocessor_token(&current_token, "std") {
                            is_std = true;
                        } else if is_preprocessor_token(&current_token, "outfile") {
                            is_outfile = true;
                        } else {
                            tokenized_data.push(current_token.clone());
                        }

                        if ch != ' ' {
                            tokenized_data.push(format!("{}", ch));
                        } 

                        current_token.clear();
                    }
                },
                '\'' |
                '"' => {
                    if escape_sequence {
                        data = true;
                        current_token.push(ch);
                    } else {
                        if !current_token.starts_with(ch) {
                            data = true;
                            current_token.push(ch);
                            continue;
                        }

                        if is_link {
                            let mut linker_flags = String::new();

                            for val in &crate::ast::ast_helpers::extract_argument(&current_token.clone()).split(' ').collect::<Vec<&str>>() {
                                linker_flags.push_str(format!("-l{} ", val).as_str());
                            }

                            linker_flags.pop();
                            tokenized_data.push(linker_flags.to_owned());
            
                            is_link = false;
                        } else if is_std {
                            tokenized_data.push(format!("-std={}",
                                                        &crate::ast::ast_helpers::extract_argument(&current_token.clone())));
            
                            is_std = false;
                        } else if is_outfile {
                            tokenized_data.push(format!("-o {}",
                                                        &crate::ast::ast_helpers::extract_argument(&current_token.clone())));
            
                            is_outfile = false;
                        } else if is_env {
                            let environment = get_environment(&crate::ast::ast_helpers::extract_argument(
                                &current_token.clone()).as_str());
            
                            // Replace whitespace with '_'
                            //if !is_data(&token) {
                                // Error (Environments must be string that has not any whitespace (use _ instead))
                            //}
            
                            if !environment.is_empty() {
                                tokenized_data.push(format!("{}", environment.to_owned()));
                            }
                            // else {
                                // Error (environment not found in this scope)
                            //}
            
                            is_env = false;
                        } else {
                            current_token.push(ch);

                            tokenized_data.push(current_token.clone());
                        }

                        data = false;
                        // escafe::run()
                        current_token.clear();
                        escape_sequence = false;
                    }
                },
                '\\' => {
                    if escape_sequence {
                        current_token.push('\\');
                    } else if data {
                        escape_sequence = true;
                    }
                },
                'n' |
                't' |
                'r' |
                'x' |
                '0' |
                '1' |
                'm' |
                'w' => {
                    if escape_sequence {
                        current_token.push('\\');
                        escape_sequence = false;
                    } current_token.push(ch);
                },
                '#' => {
                    if !escape_sequence && !data {
                        skip_until_newline = true;
                    } else {
                        current_token.push('#');
                    }
                },
                _ => {
                    current_token.push(ch);
                }
            }
        }

        if !current_token.is_empty() {
            tokenized_data.push(current_token.clone());
        }

        tokenized_data
    }

    /*fn get_data(tokens: &Vec<&str>, n: usize) -> String {
        let mut temporary = String::new();

        temporary.push_str(tokens.get(n).unwrap());

        for (_index, token) in tokens.iter().enumerate().skip(&n + 1) {
            if token.is_empty() { continue; }

            if !is_data(token) {
                temporary.push(' ');
                temporary.push_str(token);

                continue;
            }

            temporary.push_str(" ");
            temporary.push_str(token);

            break;
        }

        temporary
    }*/

    pub fn replace_for_tokenize(token: &String) -> String {
        let mut token= String::from(token);

        for character in TOKEN_LIST {
            token = replace_with(&token, *character);
        }

        token
    }

    pub fn is_data(token: &&str) -> bool {
        return if token.trim_start().starts_with('"')
            || token.trim_end().ends_with('"') {
            true
        } else { false };
    }

    pub fn is_variable(token: &&str) -> bool {
        return if token.trim_start().starts_with('$') {
            true
        } else { false };
    }

    pub fn is_preprocessor_token(token: &String, what: &str) -> bool {
        return if &token == &what {
            true
        } else { false };
    }

    pub fn is_comment(token: &&str) -> bool {
        if token.len() < 2 { return false; }

        let token = token.trim();

        return if token.starts_with('\\') && token.chars().nth(1).unwrap() == '\\' {
            true
        }
        else if token.starts_with('/') && token.chars().nth(1).unwrap() == '/' {
            elite_logger::log(EliteLogType::Warning,
                              "comment",
                              "do not use '\x1b[1;97m//\x1b[0m' as comment, ignored.");

            true
        }
        else {
            false
        };
    }

    pub fn get_environment(data: &&str) -> String {
        return match std::env::var(data) {
            Ok(__data)   => __data,
            Err(__error) => "".to_string()
        }
    }

    pub fn replace_with(token: &String, character: char) -> String {
        token.replace(character, format!(" {} ", character).as_str()).to_string()
    }
}