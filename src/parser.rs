// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use {
    std::env::var,

    crate::ast::{
        EliteKeywords,

        EliteAST,
        EliteDataTree,
        EliteDataInfos
    }
};

pub struct EliteParser {
    pub(crate) init_ast : EliteAST,
    pub(crate) data_tree: EliteDataTree
}

impl EliteParser {
    pub fn parse_tokens(&mut self, tokens: &Vec<String>) {
        let mut __matched_type: EliteKeywords = EliteKeywords::Undefined;

        let mut is_variable = false;
        let mut is_data     = false;

        let mut is_for         = false;
        let mut is_for_argument= false;

        let mut is_print = false;

        let mut is_data_initializer = false;

        let mut variable_name = String::new();
        let mut variable_data = String::new();

        for mut token in tokens {
            if token.is_empty() { continue; }

            let token = &self.init_ast.to(token.trim());

            __matched_type = *self.init_ast.match_types(token);

            match __matched_type {
                EliteKeywords::Set => {
                    is_variable = true;

                    continue;
                },
                EliteKeywords::As => {
                    if is_variable {
                        is_data_initializer = true;

                        continue;
                    }

                    // Syntax error {set}
                },
                EliteKeywords::For => {
                    is_for = true;

                    continue;
                },
                EliteKeywords::Print => {
                    is_print = true;

                    continue;
                },
                // Ignoring them.
                EliteKeywords::LeftParenthese |
                EliteKeywords::RightParenthese|
                EliteKeywords::LeftSqBracket  |
                EliteKeywords::RightSqBracket  => {},
                _ => {
                    // Built-in regular print function.
                    if is_print {
                        print!("{}", self.init_ast.extract_argument(token));

                        is_print = false;

                        continue;
                    }

                    // for signal("...")
                    if is_for {
                        if is_for_argument {
                            let token = &(self.init_ast.extract_argument(token));

                            for argument in &self.init_ast.ast_for_functions_arguments {
                                if argument == token {
                                    // TODO: Signal parser.
                                    break;
                                }
                            }

                            is_for = false;
                            is_for_argument = false;

                            continue;
                        }

                        for function in &self.init_ast.ast_for_functions {
                            if function == token {
                                is_for_argument = true;
                                break;
                            }
                        }

                        continue;
                    }

                    if is_variable {
                        if is_data_initializer {
                            is_variable = false;
                            is_data_initializer = false;

                            self.token_set(variable_name.clone(), token.clone());

                            variable_name.clear();

                            continue;
                        }

                        variable_name = token.clone();
                        continue;
                    }
                }
            }
        }
    }

    // Syntax rule:
    // use {VARIABLE_NAME} as {data}
    pub fn token_set(&mut self, variable: String, data: String) {
        self.data_tree.variable_list.push(
                EliteDataInfos {
                    __type: EliteKeywords::Set,
                    __name: variable,
                    __data: data
                }
        );
    }
}