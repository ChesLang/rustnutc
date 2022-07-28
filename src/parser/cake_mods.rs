use {
    cake::*,
    cake_derive::*,
};

pub fn gen_cake() -> Cake {
    let mut cake = Cake::new(RuleId("Main::main".to_string()));
    cake.add_module(Main::new());
    cake.add_module(Block::new());
    cake.add_module(Symbol::new());
    cake.add_module(Expression::new());
    cake.add_module(Literal::new());
    cake.add_module(Keyword::new());
    cake.add_module(Misc::new());
    cake
}

#[derive(RuleContainer)]
struct Main {
    main: Element,
}

impl Module for Main {
    fn new() -> Main {
        add_rules!{
            main := Block::block().zero_or_more();
        }
    }
}

#[derive(RuleContainer)]
struct Block {
    block: Element,
    def_function: Element,
}

impl Block {
    fn block_with_end(def: Element, content: Element) -> Element {
        (def.separate(!Misc::token_separator(), false).group() + !Misc::line_end() + content.separate(!Misc::token_separator(), false).group().name("content") + !Keyword::end()).separate(!Misc::token_separator(), false)
    }
}

impl Module for Block {
    fn new() -> Block {
        add_rules!{
            block := !Misc::whitespace_or_line_end().zero_or_more() + Block::def_function() + !Misc::whitespace_or_line_end().zero_or_more();
            def_function := Block::block_with_end(
                !str("fn") + skip_separator() + !Misc::single_token_separator() + str("test").name("name") + !str("(") + !str(")"),
                (Expression::expression() + !Misc::expression_separator().one_or_more()).zero_or_more(),
            );
        }
    }
}

#[derive(RuleContainer)]
struct Expression {
    pub expression: Element,
}

impl Module for Expression {
    fn new() -> Expression {
        add_rules!{
            expression := Literal::string();
        }
    }
}

#[derive(RuleContainer)]
struct Literal {
    string: Element,
    string_character: Element,
    escseq: Element,
}

impl Module for Literal {
    fn new() -> Literal {
        add_rules!{
            string := (str("\"").neg() + (Literal::escseq() | Literal::string_character())).zero_or_more().enclose(!str("\"")).join();
            // Add an element with neg() when escape sequence added.
            string_character := str("\\").neg() + str("\n").neg() + str("\t").neg() + wildcard();
            escseq := !str("\\") + (str("\\").replace("\\") | str("n").replace("\n") | str("t").replace("\t") | wildcard().run(Callback::on_unknown_escseq));
        }
    }
}

#[derive(RuleContainer)]
struct Symbol {
    whitespace: Element,
    new_line: Element,
    semicolon: Element,
}

impl Module for Symbol {
    fn new() -> Symbol {
        add_rules!{
            whitespace := str(" ") | str("\t");
            new_line := str("\n");
            semicolon := str(";");
        }
    }
}

#[derive(RuleContainer)]
struct Keyword {
    end: Element,
}

impl Module for Keyword {
    fn new() -> Keyword {
        add_rules!{
            end := str("end");
        }
    }
}

#[derive(RuleContainer)]
struct Misc {
    whitespace_or_line_end: Element,
    token_separator: Element,
    single_token_separator: Element,
    line_end: Element,
    expression_separator: Element,
}

impl Module for Misc {
    fn new() -> Misc {
        add_rules!{
            whitespace_or_line_end := Symbol::whitespace() | Symbol::new_line();
            token_separator := Symbol::whitespace().zero_or_more();
            single_token_separator := Symbol::whitespace();
            line_end := (Misc::token_separator() + Symbol::new_line()).one_or_more();
            expression_separator := Misc::token_separator() + (Symbol::new_line() | Symbol::semicolon()) + Misc::token_separator();
        }
    }
}

struct Callback;

impl Callback {
    fn on_unknown_escseq(_: ElementParserResult) -> ElementParserResult {
        println!("on_unknown_escseq() called.");
        Ok(Some(Vec::new()))
    }
}
