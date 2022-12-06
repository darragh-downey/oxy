use std::any::Any;

pub fn hello() {
    println!("Hello from parser")
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    #[test]
    fn let_statement() {}

    #[test]
    fn return_statement() {}

    #[test]
    fn identifier_expression() {}

    #[test]
    fn boolean_literal_expression() {
        let input = "true;";
    }

    #[test]
    fn integer_literal_expression() {
        let input = "13;";
    }

    #[test]
    fn string_literal_expression() {
        let input = "Hello World!";
    }

    #[test]
    fn if_expression() {}

    #[test]
    fn if_else_expression() {}

    #[test]
    fn function_literal() {
        let input = "fn(x) { x + y; }";
    }

    struct ParsingPrefixTestCase {
        input: String,
        operator: String,
        value: Box<dyn Any>, // First speed bump TWERP (and Go) use interface{} to handle multiple types.
                             // Is std::any::Any the right choice here?
    }

    #[test]
    fn parsing_prefix_expressions() {
        let tests: Vec<ParsingPrefixTestCase>;
    }

    struct ParsingInfixTestCase {
        input: String,
        leftValue: Box<dyn Any>,
        operator: String,
        rightValue: Box<dyn Any>, // First speed bump TWERP (and Go) use interface{} to handle multiple types.
                                  // Is std::any::Any the right choice here?
    }

    #[test]
    fn parsing_infix_expressions() {
        let tests: Vec<ParsingInfixTestCase>;
    }

    struct ExpectStringTestCase {
        input: String,
        expected: String,
    }

    #[test]
    fn operator_precedence_parsing() {
        let tests: Vec<ExpectStringTestCase>;
    }
}
