// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_lexer {
    use crate::ast::EliteDataTree;

    pub fn init_lexer(init: &crate::read::EliteFileData) {
        let tokens = crate::tokenizer::elite_tokenizer::tokenize_first(init);

        let mut init_ast = crate::ast::EliteAST {
            ast_set                    : "".to_string(),
            ast_as                     : "".to_string(),
            ast_for                    : "".to_string(),
            ast_print                  : "".to_string(),

            ast_use                    : "".to_string(),
            ast_left_parenthese        : "".to_string(),
            ast_right_parenthese       : "".to_string(),

            ast_square_left_bracket    : "".to_string(),
            ast_square_right_bracket   : "".to_string(),

            ast_for_functions          : vec![],
            ast_for_use                : vec![],

            ast_for_functions_arguments: vec![],
            ast_for_use_arguments: vec![],

            syntax_list                : Default::default(),
            ast_use_list: Default::default()
        };

        init_ast.init_keywords();

        let mut init_parser = crate::parser::EliteParser {
            init_ast : init_ast,
            data_tree: EliteDataTree { variable_list: Default::default() }
        };

        init_parser.parse_tokens(&tokens);

        //for token in tokens {
        //    if token.is_empty() || token == init_ast.to("\n") { continue; }
        //
        //    println!("<{}>", token);
        //}
    }
}