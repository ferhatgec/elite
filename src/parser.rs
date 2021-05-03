// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use {
    crate::ast::{
        EliteKeywords,

        EliteASTForFunctions,
        EliteASTForSpecificTargets,

        EliteASTIfFunctions,

        EliteASTUseArguments,
        EliteASTUseFunctions,

        EliteAST,
        EliteDataTree,
        EliteDataInfos,

        ast_helpers
    }
};

pub struct EliteParser {
    pub(crate) init_ast : EliteAST,
    pub(crate) data_tree: EliteDataTree
}

impl EliteParser {
    pub fn parse_tokens(&mut self, tokens: &Vec<String>) {
        let mut __matched_type           = EliteKeywords::Undefined;
        let mut last_matched_function    = EliteASTForFunctions::Undefined;
        let mut last_matched_if_function = EliteASTIfFunctions::Undefined;

        let mut is_variable = false;
        let mut is_defined  = false;

        let mut is_for         = false;
        let mut is_for_argument= false;

        let mut is_if          = false;
        let mut is_if_function = false;

        let mut is_print   = false;
        let mut is_newline = false;

        let mut is_use = false;
        let mut is_use_argument = false;

        let mut is_data_initializer = false;

        // Used by argument() and specific()
        let mut is_main_os  = true;

        let mut is_use_add_source   = false;
        let mut is_use_add_argument = false;

        let mut count_end_of_function: u32 = 0;

        let mut variable_name = String::new();
        let mut variable_data = String::new();

        let mut first_if_argument = String::new();
        let mut second_if_argument= String::new();

        let mut use_add_source_argument = String::new();

        let mut use_current_function = String::new();

        for token in tokens {
            if token.is_empty() { continue; }

            let mut token = self.init_ast.to(token.trim());

            if is_defined {
                token = self.token_get(token.to_owned());

                is_defined = false;
            }
            else if crate::tokenizer::elite_tokenizer::is_variable(&token.as_str()) {
                is_defined = true;

                continue;
            }

            __matched_type = *self.init_ast.match_types(&token);

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
                EliteKeywords::Println => {
                    is_print = true;
                    is_newline = true;

                    continue;
                },
                EliteKeywords::Use => {
                    is_use = true;

                    continue;
                },
                EliteKeywords::If  => {
                    is_if = true;

                    continue;
                },
                // Ignoring them.
                EliteKeywords::LeftParenthese |
                EliteKeywords::RightParenthese => {},
                EliteKeywords::LeftSqBracket   => {
                    if is_main_os  {
                        count_end_of_function += 1;
                    }

                    continue;
                },
                EliteKeywords::RightSqBracket  => {
                    if is_main_os {
                        count_end_of_function -= 1;
                    }
                    else {
                        is_main_os = true;
                    }

                    if count_end_of_function == 0 {
                        is_main_os       = false;
                    }

                    continue;
                },
                _ => {
                    if !is_main_os {
                        is_newline      = false;
                        is_print        = false;
                        is_use          = false;
                        is_use_argument = false;

                        continue;
                    }

                    if crate::tokenizer::elite_tokenizer::is_data(&token.as_str()) {
                        let mut __data   = String::new();
                        let mut __format = String::new();

                        let mut is_formatter = false;

                        for character in token.chars() {
                            if is_formatter {
                                if character == '}' {
                                    is_formatter = false;

                                    __data.push_str(self.token_get(__format.clone()).as_str());

                                    continue;
                                }

                                __format.push(character);

                                continue;
                            }

                            if character == '{' {
                                is_formatter = true;

                                continue;
                            }

                            __data.push(character);

                            continue;
                        }

                        token = __data;
                    }

                    if is_if {
                        if token.is_empty() { continue; }

                        if is_if_function {
                            if first_if_argument.is_empty() {
                                first_if_argument = crate::ast::ast_helpers::extract_argument(&token);

                                continue;
                            }

                            if second_if_argument.is_empty() {
                                second_if_argument = crate::ast::ast_helpers::extract_argument(&token);
                            }

                            match last_matched_if_function {
                                EliteASTIfFunctions::Eq |
                                EliteASTIfFunctions::UnEq => {
                                    is_main_os = self.ast_parse_if_function(variable_name.clone(),
                                                                            crate::ast::ast_helpers::extract_argument(&first_if_argument.clone()),
                                                                            crate::ast::ast_helpers::extract_argument(&second_if_argument.clone()));
                                },
                                _ => {
                                    // Syntax error
                                }
                            }

                            is_if          = false;
                            is_if_function = false;

                            first_if_argument .clear();
                            second_if_argument.clear();

                            continue;
                        }

                        if self.init_ast.match_if_functions(&token) != &EliteASTIfFunctions::Undefined {
                            is_if_function = true;

                            last_matched_if_function = *self.init_ast.match_if_functions(&token);

                            variable_name   = token.clone();
                        }

                        continue;
                    }

                    if is_use {
                        // All 'use' functions are takes 1 argument
                        // add_source is excepted, it's requires 2 argument.
                        if is_use_add_source {
                            if is_use_add_argument {
                                let token = ast_helpers::extract_argument(&token);

                                match self.init_ast.match_use_functions(&use_current_function) {
                                    EliteASTUseFunctions::AddSource => {
                                        //if !std::path::Path::new(&token).exists() {
                                        // Warning (added source file is not exists)
                                        //}

                                        self.token_append(use_add_source_argument.clone(),
                                                          ast_helpers::extract_argument(&token),
                                                          ' ');
                                    },
                                    EliteASTUseFunctions::Append    => {
                                        self.token_append(use_add_source_argument.clone(),
                                                          ast_helpers::extract_argument(&token),
                                                          Default::default());
                                    }
                                    _ => {}
                                }


                                is_use_add_source   = false;
                                is_use_add_argument = false;

                                use_add_source_argument.clear();

                                continue;
                            }

                            use_add_source_argument = token.clone();

                            is_use_add_argument     = true;

                            continue;
                        }

                        match self.init_ast.match_use_functions(&token).clone() {
                            EliteASTUseFunctions::AddSource |
                            EliteASTUseFunctions::Append     => {
                                is_use_add_source = true;

                                use_current_function = token.clone();

                                continue;
                            },
                            _ => {
                                is_use_add_source = false;

                                use_current_function = token.clone();
                            }
                        }

                        if is_use_argument {
                            let token: &String = &ast_helpers::extract_argument(&token);

                            for argument in &self.init_ast.ast_for_use_arguments.clone() {
                                if argument == token {
                                    self.ast_parse_use(token.to_string());
                                }
                            }

                            if token.is_empty() { continue; }


                            self.ast_parse_use_function(variable_data.clone(),
                                                        ast_helpers::extract_argument(token));


                            is_use = false;
                            is_use_argument = false;

                            continue;
                        }

                        for function in &self.init_ast.ast_for_use {
                            if function == &token {
                                is_use_argument = true;
                                variable_data = function.clone();

                                continue;
                            }
                        }
                    }

                    // Built-in regular print function.
                    if is_print {
                        print!("{}", ast_helpers::extract_argument(&token));

                        is_print = false;

                        if is_newline {
                            println!();

                            is_newline = false;
                        }

                        continue;
                    }

                    // for signal("...")
                    if is_for {
                        if is_for_argument {
                            let token = &(ast_helpers::extract_argument(&token));

                            match last_matched_function {
                                EliteASTForFunctions::Specific |
                                EliteASTForFunctions::Argument |
                                EliteASTForFunctions::Exists   => {
                                    is_main_os = self.ast_parse_for_functions(variable_name.clone(),
                                                                              ast_helpers::extract_argument(token));
                                },
                                _ => {
                                    self.ast_parse_for_functions(variable_name.clone(),
                                                                 ast_helpers::extract_argument(token));
                                }
                            }

                            is_for = false;
                            is_for_argument = false;

                            variable_name.clear();

                            continue;
                        }

                        if self.init_ast.match_for_functions(&token) != &EliteASTForFunctions::Undefined {
                            is_for_argument = true;
                            last_matched_function = *self.init_ast.match_for_functions(&token);

                            variable_name   = token.clone();
                        }

                        continue;
                    }

                    if is_variable {
                        if is_data_initializer {
                            is_variable = false;
                            is_data_initializer = false;

                            self.token_set(variable_name.clone(),
                                           ast_helpers::extract_argument(&token));

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

    pub fn ast_parse_for_functions(&mut self, function: String, argument: String) -> bool {
        match self.init_ast.match_for_functions(&function) {
            EliteASTForFunctions::Signal => {
                // TODO: Signal parser.
                false
            },
            EliteASTForFunctions::Specific => {
                self.ast_parse_for_specific_target(argument)
            },
            EliteASTForFunctions::Argument => {
                self.is_same_arg(&argument)
            },
            EliteASTForFunctions::Exists   => {
                self.is_exists(&argument)
            },
            _ => {
                // Syntax error (undefined function)
                false
            }
        }
    }

    pub fn ast_parse_for_specific_target(&mut self, target: String) -> bool {
        match self.init_ast.match_for_specific_targets(&target) {
            // EliteASTForSpecificTargets::Windows => {
            //
            // },
            // EliteASTForSpecificTargets::macOS => {
            //
            // },
            // EliteASTForSpecificTargets::iOS => {
            //
            // },
            // EliteASTForSpecificTargets::Linux => {
            //
            // },
            // EliteASTForSpecificTargets::Android => {
            //
            // },
            // EliteASTForSpecificTargets::FreeBSD => {
            //
            // },
            // EliteASTForSpecificTargets::DragonFly => {
            //
            // },
            // EliteASTForSpecificTargets::Bitrig => {
            //
            // },
            // EliteASTForSpecificTargets ::OpenBSD => {
            //
            // },
            // EliteASTForSpecificTargets::NetBSD => {
            //
            // },
            EliteASTForSpecificTargets::Undefined => {
                false
            },
            _ => {
                return self.is_same(&target);

                // println!("Undefined target {}", &target);
            }
        }
    }

    pub fn ast_parse_if_function(&mut self, function: String,  argument_1: String, argument_2: String) -> bool {
        match self.init_ast.match_if_functions(&function) {
            EliteASTIfFunctions::Eq => {
                self.is_same_argument(&argument_1, &argument_2)
            },
            EliteASTIfFunctions::UnEq => {
                self.is_not_same_argument(&argument_1, &argument_2)
            },
            EliteASTIfFunctions::Undefined => {
                // Syntax error (undefined function (eq, etc.)
                false
            }
        }
    }

    pub fn ast_parse_use_function(&mut self, function: String, argument: String) {
        match self.init_ast.match_use_functions(&function) {
            EliteASTUseFunctions::Signal => {
                self.ast_parse_use(argument);
            },
            EliteASTUseFunctions::Exec => {
                let mut command = String::new();

                for character in argument.split(' ').collect::<Vec<_>>().get(0).unwrap().chars() {
                    if character != '"' || character != ' ' {
                        command.push(character);

                        continue;
                    }

                    break;
                }

                if argument.contains(' ') {
                    let mut arguments: Vec<&str> = argument.split(' ').collect();

                    arguments.remove(0);

                    std::process::Command::new(command.clone())
                        .args(arguments)
                        .status()
                        .expect(format!("{} command failed to execute!", command.to_owned()).as_str());
                }
                else {
                    std::process::Command::new(command.clone())
                        .status()
                        .expect(format!("{} command failed to execute!", command.to_owned()).as_str());
                }
            }
            _ => {
                // Syntax error (Undefined function except add_source)
            }
        }
    }

    pub fn ast_parse_use(&mut self, argument: String) {
        match self.init_ast.match_use_arguments(&argument) {
            EliteASTUseArguments::Exit => {
                std::process::exit(0);
            },
            _ => {
                // Syntax error (undefined argument)
            }
        }
    }

    // Syntax rule:
    // use {VARIABLE_NAME} as {data}
    pub fn token_set(&mut self, variable: String, data: String) {
        // Check is variable exists.
        for (_index, variable_list) in self.data_tree.variable_list.iter().enumerate() {
            if variable_list.__name == variable {
                self.data_tree.variable_list[_index].__data = data.clone();

                return;
            }
        }

        self.data_tree.variable_list.push(
                EliteDataInfos {
                    __type: EliteKeywords::Set,
                    __name: variable,
                    __data: data
                }
        );
    }

    pub fn token_get(&self, variable: String) -> String {
        for variable_list in self.data_tree.variable_list.iter() {
            if variable == variable_list.__name {
                if variable_list.__type != EliteKeywords::Undefined { return String::from(variable_list.__data.clone()); }
            }
        }

        self.init_ast.to("")
    }

    pub fn token_append(&mut self, variable: String, argument: String, delimiter: char) {
        for (_index, variable_list) in self.data_tree.variable_list.iter().enumerate() {
            if variable_list.__name == variable {
                self.data_tree.variable_list[_index].__data.push_str(
                    format!("{}{}", delimiter, argument).as_str());
                return;
            }
        }
    }

    pub fn is_exists(&self, path: &String) -> bool {
        return if std::path::Path::new(path).exists() {
            true
        } else { false };
    }

    pub fn is_same_arg(&self, argument: &String) -> bool {
        let __argument: Vec<_> = std::env::args().collect();

        return if __argument.last().unwrap() == argument {
            true
        } else { false };
    }

    pub fn is_same(&self, target: &String) -> bool {
        return if std::env::consts::OS == target {
            true
        } else { false };
    }

    pub fn is_same_argument(&self, argument_1: &String, argument_2: &String) -> bool {
        return if argument_1 == argument_2 {
            true
        } else { false };
    }

    pub fn is_not_same_argument(&self, argument_1: &String, argument_2: &String) -> bool {
        return if argument_1 != argument_2 {
            true
        } else { false };
    }
}