// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub enum EliteKeywords {
    Set,
    As,
    For,
    Print,
    Println,
    Use,
    If,
    RequiredVersion,
    Suppress,

    LeftParenthese,
    RightParenthese,

    LeftSqBracket,
    RightSqBracket,

    Undefined
}

#[derive(PartialEq, Clone, Copy)]
pub enum EliteASTForFunctions {
    Signal,
    Specific,
    Argument,
    Exists,
    Undefined
}

#[allow(non_camel_case_types)]
pub enum EliteASTForSpecificTargets {
    Windows,
    macOS,
    iOS,
    Linux,
    Android,
    FreeBSD,
    DragonFly,
    Bitrig,
    OpenBSD,
    NetBSD,

    x86,
    x86_64,
    Mips,
    PowerPc,
    PowerPc64,
    Arm,
    AArch64,

    Undefined
}

#[derive(PartialEq, Copy, Clone)]
pub enum EliteASTIfFunctions {
    Eq,
    UnEq,
    Undefined
}

#[derive(PartialEq)]
pub enum EliteASTUseFunctions {
    Signal,
    Exec,
    AddSource,
    Append,
    Undefined
}

pub enum EliteASTUseArguments {
    Exit,
    Undefined
}

pub struct EliteAST {
    pub ast_set                    : String,
    pub ast_as                     : String,
    pub ast_is                     : String,
    pub ast_for                    : String,
    pub ast_print                  : String,
    pub ast_println                : String,
    pub ast_use                    : String,
    pub ast_if                     : String,
    pub ast_required_version       : String,
    pub ast_suppress               : String,

    pub ast_left_parenthese        : String,
    pub ast_right_parenthese       : String,

    pub ast_square_left_bracket    : String,
    pub ast_square_right_bracket   : String,

    pub ast_for_use                : Vec<String>,

    pub ast_for_functions_arguments: Vec<String>,
    pub ast_for_use_arguments      : Vec<String>,

    pub syntax_list                : HashMap<String, EliteKeywords>,

    pub ast_for_functions          : HashMap<String, EliteASTForFunctions>,
    pub ast_for_specific_targets   : HashMap<String, EliteASTForSpecificTargets>,

    pub ast_if_functions           : HashMap<String, EliteASTIfFunctions>,

    pub ast_use_functions          : HashMap<String, EliteASTUseFunctions>,
    pub ast_use_list               : HashMap<String, EliteASTUseArguments>
}

pub struct EliteDataInfos {
    pub __type: EliteKeywords,
    pub __name: String,
    pub __data: String
}

pub struct EliteDataTree {
    pub variable_list: Vec<EliteDataInfos>
}

pub mod ast_helpers {
    pub fn extract_argument(argument: &String) -> String {
        if !argument.starts_with('"') && !argument.ends_with('"') {
            let mut __argument = String::new();

            for character in argument.chars() {
                if character != '"' {
                    __argument.push(character);

                    continue;
                }

                break;
            }

            return __argument;
        }
        else {
            let mut argument = String::from(argument);

            if argument.chars().nth(0).unwrap() == '"' {
                argument.remove(0);
            }

            argument = argument.split('\n').collect::<Vec<_>>().get(0).unwrap().to_string();

            if argument.ends_with('"') {
                argument.remove(argument.len() - 1);
            }

            argument
        }
    }
}

impl Default for EliteAST {
    fn default() -> Self {
        EliteAST {
            ast_set                    : "".to_string(),
            ast_as                     : "".to_string(),
            ast_is                     : "".to_string(),
            ast_for                    : "".to_string(),
            ast_print                  : "".to_string(),
            ast_println                : "".to_string(),

            ast_use                    : "".to_string(),
            ast_if                     : "".to_string(),
            ast_required_version       : "".to_string(),
            ast_suppress               : "".to_string(),

            ast_left_parenthese        : "".to_string(),
            ast_right_parenthese       : "".to_string(),

            ast_square_left_bracket    : "".to_string(),
            ast_square_right_bracket   : "".to_string(),

            ast_for_use                : vec![],

            ast_for_functions_arguments: vec![],
            ast_for_use_arguments      : vec![],

            syntax_list                : Default::default(),
            ast_for_functions          : Default::default(),
            ast_for_specific_targets   : Default::default(),
            ast_if_functions           : Default::default(),
            ast_use_functions          : Default::default(),
            ast_use_list               : Default::default()
        }
    }
}

impl EliteAST {
    pub fn init_keywords(&mut self) {
        self.ast_set                 = self.to("set"             );
        self.ast_as                  = self.to("as"              );
        self.ast_is                  = self.to("is"              );
        self.ast_for                 = self.to("for"             );
        self.ast_print               = self.to("print"           );
        self.ast_println             = format!("{}ln", self.ast_print );
        self.ast_use                 = self.to("use"             );
        self.ast_if                  = self.to("if"              );
        self.ast_required_version    = self.to("required_version");
        self.ast_suppress            = self.to("suppress"        );

        self.ast_left_parenthese     = self.to("("               );
        self.ast_right_parenthese    = self.to(")"               );

        self.ast_square_left_bracket = self.to("["               );
        self.ast_square_right_bracket= self.to("]"               );

        self.ast_for_functions_arguments = vec![
            self.to("start")
        ];

        self.ast_for_use = vec![
            self.to("signal"),
            self.to("exec")
        ];

        self.ast_for_use_arguments = vec![
            self.to("exit")
        ];

        self.add_token(self.ast_set.clone    (), EliteKeywords::Set    );

        // 'as' & 'is' keywords are same however,
        // 'is' mostly using by 'required_version' to declare required version,
        // 'as' mostly using by 'set' to variable data declaration.
        self.add_token(self.ast_as.clone     (), EliteKeywords::As     );
        self.add_token(self.ast_is.clone     (), EliteKeywords::As     );
        self.add_token(self.ast_for.clone    (), EliteKeywords::For    );
        self.add_token(self.ast_print.clone  (), EliteKeywords::Print  );
        self.add_token(self.ast_println.clone(), EliteKeywords::Println);
        self.add_token(self.ast_use.clone    (), EliteKeywords::Use    );
        self.add_token(self.ast_if.clone     (), EliteKeywords::If     );
        self.add_token(self.ast_required_version
                                             .clone(),EliteKeywords::
                                                                       RequiredVersion);
        self.add_token(self.ast_suppress
                                             .clone(), EliteKeywords::
                                                                       Suppress       );

        self.add_token(self.ast_left_parenthese.clone(), EliteKeywords::LeftParenthese  );
        self.add_token(self.ast_right_parenthese.clone(), EliteKeywords::RightParenthese);

        self.add_token(self.ast_square_left_bracket.clone(), EliteKeywords::LeftSqBracket  );
        self.add_token(self.ast_square_right_bracket.clone(), EliteKeywords::RightSqBracket);

        self.add_for_function(self.to("signal"  ), EliteASTForFunctions::Signal  );
        self.add_for_function(self.to("specific"), EliteASTForFunctions::Specific);
        self.add_for_function(self.to("argument"), EliteASTForFunctions::Argument);
        self.add_for_function(self.to("exists"  ), EliteASTForFunctions::Exists  );

        self.add_for_specific_target(self.to("windows"  ), EliteASTForSpecificTargets::Windows  );
        self.add_for_specific_target(self.to("macos"    ), EliteASTForSpecificTargets::macOS    );
        self.add_for_specific_target(self.to("ios"      ), EliteASTForSpecificTargets::iOS      );
        self.add_for_specific_target(self.to("linux"    ), EliteASTForSpecificTargets::Linux    );
        self.add_for_specific_target(self.to("android"  ), EliteASTForSpecificTargets::Android  );
        self.add_for_specific_target(self.to("freebsd"  ), EliteASTForSpecificTargets::FreeBSD  );
        self.add_for_specific_target(self.to("dragonfly"), EliteASTForSpecificTargets::DragonFly);
        self.add_for_specific_target(self.to("bitrig"   ), EliteASTForSpecificTargets::Bitrig   );
        self.add_for_specific_target(self.to("openbsd"  ), EliteASTForSpecificTargets::OpenBSD  );
        self.add_for_specific_target(self.to("netbsd"   ), EliteASTForSpecificTargets::NetBSD   );

        self.add_for_specific_target(self.to("x86"      ), EliteASTForSpecificTargets::x86      );
        self.add_for_specific_target(self.to("x86_64"   ), EliteASTForSpecificTargets::x86_64   );
        self.add_for_specific_target(self.to("mips"     ), EliteASTForSpecificTargets::Mips     );
        self.add_for_specific_target(self.to("powerpc"  ), EliteASTForSpecificTargets::PowerPc  );
        self.add_for_specific_target(self.to("powerpc64"), EliteASTForSpecificTargets::PowerPc64);
        self.add_for_specific_target(self.to("arm"      ), EliteASTForSpecificTargets::Arm      );
        self.add_for_specific_target(self.to("aarch64"  ), EliteASTForSpecificTargets::AArch64  );

        self.add_if_function        (self.to("eq"      ), EliteASTIfFunctions::Eq  );
        self.add_if_function        (self.to("uneq"    ), EliteASTIfFunctions::UnEq);

        self.add_use_function(self.to("signal"    ), EliteASTUseFunctions::Signal   );
        self.add_use_function(self.to("exec"      ), EliteASTUseFunctions::Exec     );
        self.add_use_function(self.to("add_source"), EliteASTUseFunctions::AddSource);
        self.add_use_function(self.to("append"    ), EliteASTUseFunctions::Append   );

        self.add_use_argument(self.to("exit"     ), EliteASTUseArguments::Exit     );
    }

    fn add_token(&mut self, token: String, token_type: EliteKeywords) {
        self.syntax_list.insert(token, token_type);
    }

    fn add_for_function(&mut self, function: String, token_type: EliteASTForFunctions) {
        self.ast_for_functions.insert(function, token_type);
    }

    fn add_for_specific_target(&mut self, function: String, token_type: EliteASTForSpecificTargets) {
        self.ast_for_specific_targets.insert(function, token_type);
    }

    fn add_if_function(&mut self, statement: String, token_type: EliteASTIfFunctions) {
        self.ast_if_functions.insert(statement, token_type);
    }

    fn add_use_function(&mut self, function: String, token_type: EliteASTUseFunctions) {
        self.ast_use_functions.insert(function, token_type);
    }

    fn add_use_argument(&mut self, argument: String, token_type: EliteASTUseArguments) {
        self.ast_use_list.insert(argument, token_type);
    }

    pub fn to(&self, data: &str) -> String {
        data.to_string()
    }

    pub fn match_for_functions(&mut self, function: &String) -> &EliteASTForFunctions {
        let function = self.ast_for_functions.get(function);

        if function.is_none() { return &EliteASTForFunctions::Undefined; }

        function.unwrap()
    }

    pub fn match_for_specific_targets(&mut self, target: &String) -> &EliteASTForSpecificTargets {
        let target = self.ast_for_specific_targets.get(target);

        if target.is_none() { return &EliteASTForSpecificTargets::Undefined; }

        target.unwrap()
    }

    pub fn match_if_functions(&mut self, statement: &String) -> &EliteASTIfFunctions {
        let statement = self.ast_if_functions.get(statement);

        if statement.is_none() { return &EliteASTIfFunctions::Undefined; }

        statement.unwrap()
    }

    pub fn match_use_functions(&mut self, function: &String) -> &EliteASTUseFunctions {
        let function = self.ast_use_functions.get(function);

        if function.is_none() { return &EliteASTUseFunctions::Undefined; }

        function.unwrap()
    }

    pub fn match_use_arguments(&mut self, argument: &String) -> &EliteASTUseArguments {
        let argument = self.ast_use_list.get(argument);

        if argument.is_none() { return &EliteASTUseArguments::Undefined; }

        argument.unwrap()
    }

    pub fn match_types(&mut self, token: &String) -> &EliteKeywords {
        let token_type = self.syntax_list.get(token);

        if token_type.is_none() { return &EliteKeywords::Undefined; }

        token_type.unwrap()
    }
}