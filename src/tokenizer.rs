// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_tokenizer {
    static TOKEN_LIST: &'static [char] = &[
        '(',
        ')',
        '[',
        ']'
    ];

    pub fn tokenize_first(raw_data: &crate::read::EliteFileData) -> Vec<String> {
        let temporary_tokens: Vec<_> = raw_data.raw_data.split(' ').collect();
        let mut tokenized_data: Vec<String> = vec![];

        let mut variable_data : String = String::new();

        let mut found_data = false;

        for (mut index, token) in temporary_tokens.iter().enumerate() {
            if is_data(&token) {
                found_data = true;
                tokenized_data.push(get_data(&temporary_tokens, index));


                continue;
            }

            let mut token: String = String::from(replace_for_tokenize(&token.to_string()));
            let mut x: Vec<_> = token.split(' ').collect::<Vec<&str>>();

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

        for (index, token) in tokens.iter().enumerate().skip(&n + 1) {
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
        return if token.trim_start().starts_with('"') || token.trim_end().ends_with('"') {
            true
        } else { false };
    }

    pub fn is_comment(token: &&str) -> bool {
        if token.len() < 2 { return false; }

        return if token.trim_start().starts_with('\\') && token.trim_start().chars().nth(1).unwrap() == '\\' {
            true
        } else { false };
    }

    pub fn replace_with(token: &String, character: char) -> String {
        token.replace(character, format!(" {} ", character).as_str()).to_string()
    }
}