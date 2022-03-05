// MIT License
//
// Copyright (c) 2021-2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//
use {
    crate::{
        ast::{
            Branch,

            EliteKeywords,

            EliteAST,
            ASTNode,
            EliteDataTree,
            EliteDataInfos,

            ast_helpers
        },

        logger::{*},

        VALID_VERSIONS,
        VERSION
    }
};


pub struct EliteParser {
    pub init_ast : EliteAST,
    pub ast_nodes: ASTNode,
    pub data_tree: EliteDataTree,
    pub arguments: Vec<String>,
    pub platforms: Vec<String>,
    pub just_ct  : bool
}

impl EliteParser {
    pub fn parse_tokens(&mut self, tokens: &Vec<String>) {
        let mut __matched_type           = EliteKeywords::Undefined;
        let mut last_matched_function    = EliteKeywords::Undefined;
        let mut last_matched_if_function = EliteKeywords::Undefined;

        let mut is_variable                     = false;
        let mut is_defined                      = false;

        let mut is_for                          = false;
        let mut is_for_argument                 = false;

        let mut is_if                           = false;
        let mut is_if_function                  = false;

        let mut is_print                        = false;
        let mut is_newline                      = false;

        let mut is_use                          = false;
        let mut is_use_argument                 = false;

        let mut is_required_version             = false;
        let mut is_required_version_initializer = false;

        let mut is_suppress                     = false;

        let mut is_data_initializer             = false;

        // Used by argument() and specific()
        let mut is_main_os                      = true ;

        let mut is_use_add_source               = false;
        let mut is_use_add_argument             = false;

        let mut count_end_of_function: u32            = 0    ;

        let mut variable_name                  = String::new();
        let mut variable_data                  = String::new();

        let mut first_if_argument              = String::new();
        let mut second_if_argument             = String::new();

        let mut use_add_source_argument        = String::new();

        let mut use_current_function           = String::new();

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
                },
                EliteKeywords::As => {
                    if is_variable {
                        is_data_initializer             = true;

                        continue;
                    }

                    if is_required_version {
                        is_required_version_initializer = true;
                    }
                },
                EliteKeywords::For => {
                    is_for = true;
                },
                EliteKeywords::Print => {
                    is_print = true;
                },
                EliteKeywords::Println => {
                    is_print = true;
                    is_newline = true;
                },
                EliteKeywords::Use => {
                    is_use = true;
                },
                EliteKeywords::If  => {
                    is_if = true;
                },
                EliteKeywords::RequiredVersion => {
                    is_required_version = true;
                },
                EliteKeywords::Suppress => {
                    is_suppress = true;
                },

                // Ignoring them.
                EliteKeywords::LeftParenthese |
                EliteKeywords::RightParenthese => {},

                EliteKeywords::LeftSqBracket   => {
                    self.ast_nodes.insert_key(EliteDataInfos {
                        __type: EliteKeywords::LeftSqBracket,
                        __name: Default::default(),
                        __data: Default::default()
                    }, Branch::Data);

                    if is_main_os  {
                        count_end_of_function += 1;
                    }

                    continue;
                },
                EliteKeywords::RightSqBracket  => {
                    self.ast_nodes.insert_key(EliteDataInfos {
                        __type: EliteKeywords::RightSqBracket,
                        __name: Default::default(),
                        __data: Default::default()
                    }, Branch::Data);

                    if count_end_of_function == 0 {
                        elite_logger::log(EliteLogType::Warning,
                                          "right-sq-bracket",
                                          "unmatched bracket");
                    }

                    if is_main_os {
                        count_end_of_function -= 1;
                    }
                    else {
                        is_main_os = true;
                    }

                    if count_end_of_function == 0 {
                        is_main_os       = false;
                    }
                },
                _ => {
                    if !is_main_os && !self.just_ct {
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

                                    __format.clear();

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
                                EliteKeywords::Eq |
                                EliteKeywords::UnEq => {
                                    self.ast_nodes.insert_key(EliteDataInfos {
                                        __type: EliteKeywords::IfArg,
                                        __name: variable_name.clone(),
                                        __data: crate::ast::ast_helpers::extract_argument(&first_if_argument.clone())
                                    }, Branch::Data);

                                    self.ast_nodes.insert_key(EliteDataInfos {
                                        __type: last_matched_if_function.clone(),
                                        __name: variable_name.clone(),
                                        __data: crate::ast::ast_helpers::extract_argument(&second_if_argument.clone())
                                    }, Branch::Data);

                                    is_main_os = self.ast_parse_if_function(variable_name.clone(),
                                                                            crate::ast::ast_helpers::extract_argument(&first_if_argument.clone()),
                                                                            crate::ast::ast_helpers::extract_argument(&second_if_argument.clone()));
                                },
                                _ => {
                                    elite_logger::log(EliteLogType::Error,
                                                      &token,
                                                      "syntax error, \
                                                        undefined if function");
                                }
                            }

                            is_if          = false;
                            is_if_function = false;

                            first_if_argument .clear();
                            second_if_argument.clear();

                            continue;
                        }

                        if self.init_ast.match_if_functions(&token) != &EliteKeywords::Undefined {
                            is_if_function = true;

                            last_matched_if_function = *self.init_ast.match_if_functions(&token);

                            variable_name   = token.clone();
                        }
                        else {
                            elite_logger::log(EliteLogType::Error,
                                        &token, "syntax error, undefined if function");
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
                                    EliteKeywords::AddSource => {
                                        if !std::path::Path::new(&token).exists() {
                                            elite_logger::log(EliteLogType::Warning,
                                                              &token,  "source file is not exist");
                                        }

                                        self.token_append(use_add_source_argument.clone(),
                                                          ast_helpers::extract_argument(&token),
                                                          ' ');
                                    },
                                    EliteKeywords::Append    => {
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
                            EliteKeywords::AddSource |
                            EliteKeywords::Append     => {
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
                                                        ast_helpers::extract_argument(token),
                                                        is_suppress);

                            if is_suppress { is_suppress = false; }

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
                        if !is_suppress {
                            if !self.just_ct {
                                print!("{}",
                                       ast_helpers::extract_argument(&ast_helpers::extract_argument(&token)));
                            }
                        }

                        is_print = false;

                        if is_newline {
                            self.ast_nodes.insert_key(EliteDataInfos {
                                __type: EliteKeywords::Println,
                                __name: "not_defined".to_string(),
                                __data: ast_helpers::extract_argument(&ast_helpers::extract_argument(&token))
                            }, Branch::Data);

                            if !is_suppress {
                                if !self.just_ct {
                                    println!();
                                }
                            }

                            is_newline = false;
                        } else {
                            self.ast_nodes.insert_key(EliteDataInfos {
                                __type: EliteKeywords::Print,
                                __name: "not_defined".to_string(),
                                __data: ast_helpers::extract_argument(&ast_helpers::extract_argument(&token))
                            }, Branch::Data);
                        }

                        is_suppress = false;
                        continue;
                    }

                    // for signal("...")
                    if is_for {
                        if is_for_argument {
                            let token = &(ast_helpers::extract_argument(&token));

                            match last_matched_function {
                                EliteKeywords::Specific |
                                EliteKeywords::Argument |
                                EliteKeywords::Exists => {
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

                        if self.init_ast.match_for_functions(&token) != &EliteKeywords::Undefined {
                            is_for_argument = true;
                            last_matched_function = *self.init_ast.match_for_functions(&token);

                            variable_name   = token.clone();
                        }

                        continue;
                    }

                    if is_required_version {
                        if is_required_version_initializer {
                            self.ast_nodes.insert_key(EliteDataInfos {
                                __type: EliteKeywords::RequiredVersion,
                                __name: token.clone(),
                                __data: token.clone()
                            }, Branch::Data);

                            if crate::tokenizer::elite_tokenizer::is_data(&token.as_str()) {
                                elite_logger::log(EliteLogType::Error,
                                                  "string_notation", &*format!(
                                                      "required_version must be used with \
                                                      float literal, \
                                                      not string. \
                                                      use {} instead of \"{data}\"",
                                                            data = ast_helpers::extract_argument(&token)));
                            }

                            is_required_version             = false;
                            is_required_version_initializer = false;

                            let version       = token.parse::<f32>().unwrap();
                            let mut is_valid = false;

                            for __version in VALID_VERSIONS {
                                // Valid version
                                if __version == &version {
                                    if version > VERSION {
                                        elite_logger::log(EliteLogType::Info,
                                                          "required_version",
                                                          &*format!("required {} or latest version",
                                                                    token));

                                        if !self.just_ct {
                                            std::process::exit(1);
                                        }
                                    }

                                    is_valid = true;

                                    break;
                                }
                            }

                            if !is_valid {
                                elite_logger::log(EliteLogType::Error,
                                                  "invalid_version",
                                                  &*format!("invalid version: {}, \
                                                        for latest valid version: {}",
                                                            version,
                                                            VALID_VERSIONS.last().unwrap()));
                            }
                        }
                    }

                    if is_variable {
                        if is_data_initializer {
                            is_variable         = false;
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
        self.ast_nodes.insert_key(EliteDataInfos {
            __type: self.init_ast.match_for_functions(&function).clone(),
            __name: "not_defined".to_string(),
            __data: argument.clone()
        }, Branch::Data);

        match *self.init_ast.match_for_functions(&function) {
            EliteKeywords::Argument => {
                if !self.arguments.contains(&argument.clone()) {
                    self.arguments.push(argument.clone());
                }
            },
            EliteKeywords::Specific => {
                if !self.platforms.contains(&argument.clone()) {
                    self.platforms.push(argument.clone());
                }
            }, _ => {}
        } if self.just_ct { return true; }

        match self.init_ast.match_for_functions(&function) {
            EliteKeywords::Signal => {
                // TODO: Signal parser.
                false
            },
            EliteKeywords::Specific => {
                self.ast_parse_for_specific_target(argument)
            },
            EliteKeywords::Argument => {
                self.is_same_arg(&argument)
            },
            EliteKeywords::Exists   => {
                self.is_exists(&argument)
            },
            _ => {
                elite_logger::log(EliteLogType::Error,
                                  &function,
                                  "syntax error, undefined for function");
                false
            }
        }
    }

    pub fn ast_parse_for_specific_target(&mut self, target: String) -> bool {
        if self.just_ct { return true; }
        match self.init_ast.match_for_specific_targets(&target) {
            // EliteKeywords::Windows => {
            //
            // },
            // EliteKeywords::macOS => {
            //
            // },
            // EliteKeywords::iOS => {
            //
            // },
            // EliteKeywords::Linux => {
            //
            // },
            // EliteKeywords::Android => {
            //
            // },
            // EliteKeywords::FreeBSD => {
            //
            // },
            // EliteKeywords::DragonFly => {
            //
            // },
            // EliteKeywords::Bitrig => {
            //
            // },
            // EliteKeywords ::OpenBSD => {
            //
            // },
            // EliteKeywords::NetBSD => {
            //
            // },
            EliteKeywords::Undefined => {
                elite_logger::log(EliteLogType::Error,
                                  &target,
                                  "undefined os target");

                false
            },
            _ => {
                return self.is_same(&target);
            }
        }
    }

    pub fn ast_parse_if_function(&mut self, function: String,  argument_1: String, argument_2: String) -> bool {
        if self.just_ct { return true; }

        match self.init_ast.match_if_functions(&function) {
            EliteKeywords::Eq => {
                self.is_same_argument(&argument_1, &argument_2)
            },
            EliteKeywords::UnEq => {
                self.is_not_same_argument(&argument_1, &argument_2)
            },
            _ => {
                false
            }
        }
    }

    pub fn ast_parse_use_function(&mut self, function: String, argument: String, suppress: bool) {
        self.ast_nodes.insert_key(EliteDataInfos {
            __type: self.init_ast.match_use_functions(&function).clone(),
            __name: argument.clone(),
            __data: function.clone()
        }, Branch::Data);

        match self.init_ast.match_use_functions(&function) {
            EliteKeywords::Signal => {
                self.ast_parse_use(argument);
            },
            EliteKeywords::Exec => {
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

                    let mut syscall = std::process::Command::new(command.clone());
                    syscall.args(arguments);

                    if !suppress {
                        if !self.just_ct {
                            syscall.status()
                                .expect(format!("{} command failed to execute!", command.to_owned()).as_str());
                        }
                    } else {
                        match syscall.output() { _ => {} }
                    }
                }
                else {
                    let mut syscall = std::process::Command::new(command.clone());

                    if !suppress {
                        if !self.just_ct {
                            syscall.status()
                                .expect(format!("{} command failed to execute!", command.to_owned()).as_str());
                        }
                    } else {
                        match syscall.output() { _ => {} }
                    }
                }
            }
            _ => {
                elite_logger::log(EliteLogType::Error,
                                  &function,
                                  "syntax error, undefined use function");
            }
        }
    }

    pub fn ast_parse_use(&mut self, argument: String) {
        match self.init_ast.match_use_arguments(&argument) {
            EliteKeywords::Exit => {
                if !self.just_ct {
                    std::process::exit(0);
                }
            },
            _ => {
                if argument != "start" {
                    elite_logger::log(EliteLogType::Error,
                                      &format!("{}", argument),
                                      "syntax error, undefined use function");
                }
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

                self.ast_nodes.insert_key(EliteDataInfos {
                    __type: EliteKeywords::Change,
                    __name: self.data_tree.variable_list[_index].__name.clone(),
                    __data:  self.data_tree.variable_list[_index].__data.clone()
                }, Branch::Data);

                return;
            }
        }

        self.data_tree.variable_list.push(
                EliteDataInfos {
                    __type: EliteKeywords::Set,
                    __name: variable.clone(),
                    __data: data.clone()
                }
        );

        self.ast_nodes.insert_key(EliteDataInfos {
            __type: EliteKeywords::Set,
            __name: variable.clone(),
            __data: data.clone()
        }, Branch::Data);
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

                self.ast_nodes.insert_key(EliteDataInfos {
                    __type: EliteKeywords::Append,
                    __name: self.data_tree.variable_list[_index].__name.clone(),
                    __data: argument.clone()
                }, Branch::Data);

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
        return if std::env::consts::OS == target || std::env::consts::ARCH == target {
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