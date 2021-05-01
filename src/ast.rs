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
    Use,

    LeftParenthese,
    RightParenthese,

    LeftSqBracket,
    RightSqBracket,

    Undefined
}

pub enum EliteASTUseArguments {
    Exit,
    Undefined
}

pub struct EliteAST {
    pub ast_set  : String,
    pub ast_as   : String,
    pub ast_for  : String,
    pub ast_print: String,
    pub ast_use  : String,

    pub ast_left_parenthese : String,
    pub ast_right_parenthese: String,

    pub ast_square_left_bracket  : String,
    pub ast_square_right_bracket : String,

    pub ast_for_functions: Vec<String>,
    pub ast_for_use      : Vec<String>,

    pub ast_for_functions_arguments: Vec<String>,
    pub ast_for_use_arguments      : Vec<String>,

    pub syntax_list: HashMap<String, EliteKeywords>,
    pub ast_use_list:HashMap<String, EliteASTUseArguments>
}

pub struct EliteDataInfos {
    pub __type: EliteKeywords,
    pub __name: String,
    pub __data: String
}

pub struct EliteDataTree {
    pub variable_list: Vec<EliteDataInfos>
}

impl EliteAST {
    pub fn init_keywords(&mut self) {
        self.ast_set   = self.to("set"  );
        self.ast_as    = self.to("as"   );
        self.ast_for   = self.to("for"  );
        self.ast_print = self.to("print");
        self.ast_use   = self.to("use"  );

        self.ast_left_parenthese = self.to("(");
        self.ast_right_parenthese= self.to(")");

        self.ast_square_left_bracket = self.to("[");
        self.ast_square_right_bracket= self.to("]");

        self.ast_for_functions = vec![
            self.to("signal")
        ];

        self.ast_for_functions_arguments = vec![
            self.to("start")
        ];

        self.ast_for_use = self.ast_for_functions.clone();

        self.ast_for_use_arguments = vec![
            self.to("exit")
        ];

        self.add_token(self.ast_set.clone  (), EliteKeywords::Set  );
        self.add_token(self.ast_as.clone   (), EliteKeywords::As   );
        self.add_token(self.ast_for.clone  (), EliteKeywords::For  );
        self.add_token(self.ast_print.clone(), EliteKeywords::Print);
        self.add_token(self.ast_use.clone  (), EliteKeywords::Use  );

        self.add_token(self.ast_left_parenthese.clone(), EliteKeywords::LeftParenthese);
        self.add_token(self.ast_right_parenthese.clone(), EliteKeywords::RightParenthese);

        self.add_token(self.ast_square_left_bracket.clone(), EliteKeywords::LeftSqBracket);
        self.add_token(self.ast_square_right_bracket.clone(), EliteKeywords::RightSqBracket);

        self.add_use_argument(self.to("exit"), EliteASTUseArguments::Exit);
    }

    fn add_token(&mut self, token: String, token_type: EliteKeywords) {
        self.syntax_list.insert(token, token_type);
    }

    fn add_use_argument(&mut self, argument: String, token_type: EliteASTUseArguments) {
        self.ast_use_list.insert(argument, token_type);
    }

    pub fn to(&self, data: &str) -> String {
        data.to_string()
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

    pub fn extract_argument(&self, argument: &String) -> String {
        if argument.starts_with('"') && argument.ends_with('"') {
            let mut temporary = argument.chars();

            temporary.next(); temporary.next_back();

            self.to(temporary.as_str())
        }  else { return self.to(""); }
    }
}