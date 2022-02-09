// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use {
    std::collections::HashMap,
    crate::ast::EliteKeywords::{*}
};

#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(non_camel_case_types)]
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
    Change,

    IfArg,

    LeftParenthese,
    RightParenthese,

    LeftSqBracket,
    RightSqBracket,

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

    Eq,
    UnEq,

    Signal,
    Exec,
    AddSource,
    Append,

    Exit,

    Specific,
    Argument,
    Exists,

    Undefined
}

#[derive(Copy, Clone)]
pub enum Branch {
    Left,
    Right,
    Data
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

    pub ast_for_functions          : HashMap<String, EliteKeywords>,
    pub ast_for_specific_targets   : HashMap<String, EliteKeywords>,

    pub ast_if_functions           : HashMap<String, EliteKeywords>,

    pub ast_use_functions          : HashMap<String, EliteKeywords>,
    pub ast_use_list               : HashMap<String, EliteKeywords>
}

pub struct EliteDataInfos {
    pub __type: EliteKeywords,
    pub __name: String,
    pub __data: String
}

pub struct EliteDataTree {
    pub variable_list: Vec<EliteDataInfos>
}


pub struct ASTNode {
    pub data   : Vec<EliteDataInfos>,
    pub right  : Option<Box<ASTNode>>,
    pub left   : Option<Box<ASTNode>>
}

impl Default for EliteDataInfos {
    fn default() -> Self {
        EliteDataInfos {
            __type: EliteKeywords::Undefined,
            __name: "".to_string(),
            __data: "".to_string()
        }
    }
}

impl Default for ASTNode {
    fn default() -> Self {
        ASTNode {
            data: Default::default(),
            right: None,
            left: None
        }
    }
}

impl ASTNode {
    pub
    fn search(&mut self, branch: Branch) {
        let mut node = self;
        let mut x: u32 = 0;

        loop {
            x += 1;

            match branch {
                Branch::Left => {
                    if node.left.is_none() {
                        break;
                    } else {
                        for val in &node.data {
                            println!("(left){}name: {}, type: {:?}, data: {}",
                                     " ".repeat((x - 1) as usize), val.__name, val.__type, val.__data);
                        }

                        node = node.left.as_mut().unwrap();
                    }
                },
                Branch::Right => {
                    if node.right.is_none() {
                        break;
                    } else {
                        for val in &node.data {
                            println!("(right){}name: {}, type: {:?}, data: {}",
                                     " ".repeat((x - 1) as usize), val.__name, val.__type, val.__data);
                        }

                        node = node.right.as_mut().unwrap();
                    }
                },
                Branch::Data => {
                    for line in &node.data {
                        println!("(data)name: {}, type: \x1b[0;35m{:?}\x1b[0m, data: {}",
                                 if line.__name == "" {
                                     "not_defined".to_string()
                                 } else { line.__name.clone() }, line.__type, if line.__data == "" {
                                "not_defined".to_string()
                            } else { line.__data.clone() });
                    }

                    break;
                }
            }
        }
    }

    pub
    fn get_last_node(&mut self, branch: Branch) -> &ASTNode {
        let mut node = self;

        loop {
            match branch {
                Branch::Left => {
                    if node.left.is_none() {
                        break;
                    } else {
                        node = node.left.as_mut().unwrap();
                    }
                }
                Branch::Right => {
                    if node.right.is_none() {
                        break;
                    } else {
                        node = node.right.as_mut().unwrap();
                    }
                }
                Branch::Data => {
                    break;
                }
            }
        }

        node
    }

    pub(crate)
    fn ret(&mut self) -> &mut ASTNode {
        self
    }

    //pub(crate)
    //fn get_last_node_with(&mut self, branch: Branch) -> EliteKeywords {
    //    self.get_last_node(branch).data.last().unwrap().__type
    //}

    // some assignments were not used inside of function but
    // that are will be used later.
    #[allow(unused_must_use)]
    pub fn insert_key(&mut self, info: EliteDataInfos, branch: Branch) {
        let mut node = self.ret();
        let mut x: u32 = 0;

        loop {
            match branch {
                Branch::Left => {
                    if node.left.is_none() {
                        match info.__type {
                            Println |
                            Print |
                            Set |
                            Use => {
                                if x == 0 {
                                    if node.left.is_none() { node.left.insert(Default::default()); }

                                    node.data.push(EliteDataInfos {
                                        __type: info.__type,
                                        __name: info.__name.clone(),
                                        __data: info.__data.clone()
                                    });
                                    break;
                                }

                                let mut var = self.ret();

                                for _ in 0..x - 1 {
                                    var = var.left.as_mut().unwrap();
                                }

                                var.data.push(EliteDataInfos {
                                    __type: info.__type,
                                    __name: info.__name.clone(),
                                    __data: info.__data.clone()
                                });
                            },
                            _ => {
                                if node.left.is_none() { node.left.insert(Default::default()); }

                                node.left.as_mut().unwrap().data.push(EliteDataInfos {
                                    __type: info.__type,
                                    __name: info.__name.clone(),
                                    __data: info.__data.clone()
                                });
                            }
                        }

                        break;
                    } else {
                        node = node.left.as_mut().unwrap();
                        x += 1;
                    }
                },
                Branch::Right => {
                    if node.right.is_none() {
                        match info.__type {
                            IfArg |
                            Eq |
                            UnEq |
                            RightSqBracket |
                            LeftSqBracket |
                            Signal |
                            Specific |
                            Argument |
                            Exists  => {
                                if self.right.is_none() || x == 0 {
                                    if self.right.is_none() { self.right.insert(Default::default()); }

                                    self.right.as_mut().unwrap().data.push(EliteDataInfos {
                                        __type: info.__type,
                                        __name: info.__name.clone(),
                                        __data: info.__data.clone()
                                    });
                                    break;
                                }

                                let mut var = self.ret();

                                for _ in 0..x - 1 {
                                    var = var.right.as_mut().unwrap();
                                }

                                var.data.push(EliteDataInfos {
                                    __type: info.__type,
                                    __name: info.__name.clone(),
                                    __data: info.__data.clone()
                                });
                            },
                            _ => {
                                if node.right.is_none() { node.right.insert(Default::default()); }

                                node.right.as_mut().unwrap().data.push(EliteDataInfos {
                                    __type: info.__type,
                                    __name: info.__name.clone(),
                                    __data: info.__data.clone()
                                });
                            }
                        }
                        break;
                    } else {
                        node = node.right.as_mut().unwrap();
                        x += 1;
                    }
                },
                Branch::Data => {
                    node.data.push(EliteDataInfos {
                        __type: info.__type,
                        __name: info.__name.clone(),
                        __data: info.__data.clone()
                    });

                    break;
                }
            }
        }
    }
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

        self.add_for_function(self.to("signal"  ), EliteKeywords::Signal  );
        self.add_for_function(self.to("specific"), EliteKeywords::Specific);
        self.add_for_function(self.to("argument"), EliteKeywords::Argument);
        self.add_for_function(self.to("exists"  ), EliteKeywords::Exists  );

        self.add_for_specific_target(self.to("windows"  ), EliteKeywords::Windows  );
        self.add_for_specific_target(self.to("macos"    ), EliteKeywords::macOS    );
        self.add_for_specific_target(self.to("ios"      ), EliteKeywords::iOS      );
        self.add_for_specific_target(self.to("linux"    ), EliteKeywords::Linux    );
        self.add_for_specific_target(self.to("android"  ), EliteKeywords::Android  );
        self.add_for_specific_target(self.to("freebsd"  ), EliteKeywords::FreeBSD  );
        self.add_for_specific_target(self.to("dragonfly"), EliteKeywords::DragonFly);
        self.add_for_specific_target(self.to("bitrig"   ), EliteKeywords::Bitrig   );
        self.add_for_specific_target(self.to("openbsd"  ), EliteKeywords::OpenBSD  );
        self.add_for_specific_target(self.to("netbsd"   ), EliteKeywords::NetBSD   );

        self.add_for_specific_target(self.to("x86"      ), EliteKeywords::x86      );
        self.add_for_specific_target(self.to("x86_64"   ), EliteKeywords::x86_64   );
        self.add_for_specific_target(self.to("mips"     ), EliteKeywords::Mips     );
        self.add_for_specific_target(self.to("powerpc"  ), EliteKeywords::PowerPc  );
        self.add_for_specific_target(self.to("powerpc64"), EliteKeywords::PowerPc64);
        self.add_for_specific_target(self.to("arm"      ), EliteKeywords::Arm      );
        self.add_for_specific_target(self.to("aarch64"  ), EliteKeywords::AArch64  );

        self.add_if_function        (self.to("eq"      ), EliteKeywords::Eq  );
        self.add_if_function        (self.to("uneq"    ), EliteKeywords::UnEq);

        self.add_use_function(self.to("signal"    ), EliteKeywords::Signal   );
        self.add_use_function(self.to("exec"      ), EliteKeywords::Exec     );
        self.add_use_function(self.to("add_source"), EliteKeywords::AddSource);
        self.add_use_function(self.to("append"    ), EliteKeywords::Append   );

        self.add_use_argument(self.to("exit"     ), EliteKeywords::Exit     );
    }

    fn add_token(&mut self, token: String, token_type: EliteKeywords) {
        self.syntax_list.insert(token, token_type);
    }

    fn add_for_function(&mut self, function: String, token_type: EliteKeywords) {
        self.ast_for_functions.insert(function, token_type);
    }

    fn add_for_specific_target(&mut self, function: String, token_type: EliteKeywords) {
        self.ast_for_specific_targets.insert(function, token_type);
    }

    fn add_if_function(&mut self, statement: String, token_type: EliteKeywords) {
        self.ast_if_functions.insert(statement, token_type);
    }

    fn add_use_function(&mut self, function: String, token_type: EliteKeywords) {
        self.ast_use_functions.insert(function, token_type);
    }

    fn add_use_argument(&mut self, argument: String, token_type: EliteKeywords) {
        self.ast_use_list.insert(argument, token_type);
    }

    pub fn to(&self, data: &str) -> String {
        data.to_string()
    }

    pub fn match_for_functions(&mut self, function: &String) -> &EliteKeywords {
        let function = self.ast_for_functions.get(function);

        if function.is_none() { return &EliteKeywords::Undefined; }

        function.unwrap()
    }

    pub fn match_for_specific_targets(&mut self, target: &String) -> &EliteKeywords {
        let target = self.ast_for_specific_targets.get(target);

        if target.is_none() { return &EliteKeywords::Undefined; }

        target.unwrap()
    }

    pub fn match_if_functions(&mut self, statement: &String) -> &EliteKeywords {
        let statement = self.ast_if_functions.get(statement);

        if statement.is_none() { return &EliteKeywords::Undefined; }

        statement.unwrap()
    }

    pub fn match_use_functions(&mut self, function: &String) -> &EliteKeywords {
        let function = self.ast_use_functions.get(function);

        if function.is_none() { return &EliteKeywords::Undefined; }

        function.unwrap()
    }

    pub fn match_use_arguments(&mut self, argument: &String) -> &EliteKeywords {
        let argument = self.ast_use_list.get(argument);

        if argument.is_none() { return &EliteKeywords::Undefined; }

        argument.unwrap()
    }

    pub fn match_types(&mut self, token: &String) -> &EliteKeywords {
        let token_type = self.syntax_list.get(token);

        if token_type.is_none() { return &EliteKeywords::Undefined; }

        token_type.unwrap()
    }
}