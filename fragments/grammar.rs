use std::collections::HashMap;

let tatsu_grammar = Grammar {
    name: "TatSu".to_string(),
    directives: HashMap::from([
        ("grammar".to_string(), "TatSu".into()),
        ("whitespace".to_string(), r"(?m)\s+".into()),
        ("comments".to_string(), r"(?ms)[(][*]\s*((?:.|\n)*?)\s*[*][)]|[\/][*]\s*((?:.|\n)*?)\s*[*][\/]".into()),
        ("eol_comments".to_string(), r"(?ms)(?:[#]|[\/\/])(.*?)$".into()),
        ("parseinfo".to_string(), true.into()),
        ("left_recursion".to_string(), false.into()),
    ]),
    keywords: vec![],
    rules: vec![
        Rule {
            name: "start".to_string(),
            exp: Box::new(Call::new("grammar")),
            params: vec![],
            kwparams: HashMap::new(),
            decorators: vec![],
            is_name: false,
            is_leftrec: false,
            is_memoizable: true,
        },
        Rule {
            name: "grammar".to_string(),
            exp: Box::new(Sequence::new(vec![
                Box::new(Named::new("title", Box::new(Constant::new("TATSU")))),
                Box::new(Closure::new(Box::new(Choice::new(vec![
                    Box::new(Option::new(Box::new(NamedList::new("directives", Box::new(Call::new("directive")))))),
                    Box::new(Option::new(Box::new(NamedList::new("keywords", Box::new(Call::new("keyword")))))),
                ])))),
                Box::new(NamedList::new("rules", Box::new(Call::new("rule")))),
                Box::new(Closure::new(Box::new(Choice::new(vec![
                    Box::new(Option::new(Box::new(NamedList::new("rules", Box::new(Call::new("rule")))))),
                    Box::new(Option::new(Box::new(NamedList::new("keywords", Box::new(Call::new("keyword")))))),
                ])))),
                Box::new(EOF::new()),
            ])),
            params: vec!["Grammar".to_string()],
            kwparams: HashMap::new(),
            decorators: vec![],
            is_name: false,
            is_leftrec: false,
            is_memoizable: true,
        },
    ],
};
