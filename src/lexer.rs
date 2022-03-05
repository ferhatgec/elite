// MIT License
//
// Copyright (c) 2021-2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_lexer {
    use crate::ast::EliteDataTree;

    pub fn init_lexer(init: &crate::read::EliteFileData, just_create_tree: bool) -> crate::parser::EliteParser {
        let tokens = crate::tokenizer::elite_tokenizer::tokenize_first(init);

        let mut init_ast = crate::ast::EliteAST::default();

        init_ast.init_keywords();

        let mut init_parser = crate::parser::EliteParser {
            init_ast : init_ast,
            ast_nodes: Default::default(),
            data_tree: EliteDataTree { variable_list: Default::default() },
            arguments: vec![],
            platforms: vec![],
            just_ct  : just_create_tree
        };

        init_parser.parse_tokens(&tokens);

        //for token in tokens {
        //    if token.is_empty() || token == init_ast.to("\n") { continue; }
        //
        //    println!("<{}>", token);
        //}

        init_parser
    }
}