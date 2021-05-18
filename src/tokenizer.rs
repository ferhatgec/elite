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
        let temporary_tokens: Vec<_> = raw_data.raw_data.split(' ').collect();
        let mut tokenized_data: Vec<String> = vec![];

        let mut variable_data : String = String::new();

        let mut is_env     = false;
        let mut is_link    = false;

        for (_index, token) in temporary_tokens.iter().enumerate() {
            if is_env {
                let environment = get_environment(&crate::ast::ast_helpers::extract_argument(
                    &token.to_string()).as_str());

                // Replace whitespace with '_'
                //if !is_data(&token) {
                    // Error (Environments must be string that has not any whitespaces (use _ instead))
                //}

                if !environment.is_empty() {
                    tokenized_data.push(environment.to_owned());
                }
                // else {
                    // Error (environment not found in this scope)
                //}

                is_env = false;

                continue
            }

            if is_link {
                tokenized_data.push(format!("-l{}",
                                            &crate::ast::ast_helpers::extract_argument(&token.to_string())));

                is_link= false;

                continue;
            }

            if is_preprocessor_token(&token, "env") {
                is_env = true;
                continue;
            }

            if is_preprocessor_token(&token, "link") {
                is_link= true;
                continue;
            }

            if is_data(&token) {
                tokenized_data.push(get_data(&temporary_tokens, _index));

                continue;
            }

            let token: String = String::from(replace_for_tokenize(&token.to_string()));
            let x: Vec<_> = token.split(' ').collect::<Vec<&str>>();

            for operators in x {
                variable_data.push_str(operators);

                tokenized_data.push(operators.to_string());
            }
        }

        tokenized_data
    }

    fn get_data(tokens: &Vec<&str>, n: usize) -> String {
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
    }

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

    pub fn is_preprocessor_token(token: &&str, what: &str) -> bool {
        return if token == &what {
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