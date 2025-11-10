mod tests {
    use w::ast::{Expression, LogLevel};
    use w::parser::Parser;

    #[test]
    fn test_log_debug_parsing() {
        let mut parser = Parser::new("LogDebug[\"Debug message\"]".to_string());
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::LogCall { level, message } => {
                assert_eq!(level, LogLevel::Debug);
                match *message {
                    Expression::String(msg) => assert_eq!(msg, "Debug message"),
                    _ => panic!("Expected string message"),
                }
            }
            _ => panic!("Expected LogCall expression"),
        }
    }

    #[test]
    fn test_log_info_parsing() {
        let mut parser = Parser::new("LogInfo[\"Info message\"]".to_string());
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::LogCall { level, message } => {
                assert_eq!(level, LogLevel::Info);
                match *message {
                    Expression::String(msg) => assert_eq!(msg, "Info message"),
                    _ => panic!("Expected string message"),
                }
            }
            _ => panic!("Expected LogCall expression"),
        }
    }

    #[test]
    fn test_log_warn_parsing() {
        let mut parser = Parser::new("LogWarn[\"Warning message\"]".to_string());
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::LogCall { level, message } => {
                assert_eq!(level, LogLevel::Warn);
                match *message {
                    Expression::String(msg) => assert_eq!(msg, "Warning message"),
                    _ => panic!("Expected string message"),
                }
            }
            _ => panic!("Expected LogCall expression"),
        }
    }

    #[test]
    fn test_log_error_parsing() {
        let mut parser = Parser::new("LogError[\"Error message\"]".to_string());
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::LogCall { level, message } => {
                assert_eq!(level, LogLevel::Error);
                match *message {
                    Expression::String(msg) => assert_eq!(msg, "Error message"),
                    _ => panic!("Expected string message"),
                }
            }
            _ => panic!("Expected LogCall expression"),
        }
    }

    #[test]
    fn test_log_with_non_string_message() {
        let mut parser = Parser::new("LogInfo[42]".to_string());
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::LogCall { level, message } => {
                assert_eq!(level, LogLevel::Info);
                match *message {
                    Expression::Number(num) => assert_eq!(num, 42),
                    _ => panic!("Expected number message"),
                }
            }
            _ => panic!("Expected LogCall expression"),
        }
    }

    // New tests for Cond expression parsing
    #[test]
    fn test_cond_single_condition() {
        let mut parser = Parser::new("Cond[[x > 10 Print[\"Greater than 10\"]]]".to_string());
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Cond { conditions, default_statements } => {
                assert_eq!(conditions.len(), 0);
                assert!(default_statements.is_some());
                
                match *default_statements.unwrap() {
                    Expression::FunctionCall { 
                        function: func, 
                        arguments 
                    } => {
                        match *func {
                            Expression::Identifier(name) => assert_eq!(name, "Print"),
                            _ => panic!("Expected Print function"),
                        }
                        assert_eq!(arguments.len(), 1);
                        match arguments[0] {
                            Expression::String(ref msg) => assert_eq!(msg, "Greater than 10"),
                            _ => panic!("Expected string argument"),
                        }
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected Cond expression"),
        }
    }

    #[test]
    fn test_cond_multiple_conditions() {
        let mut parser = Parser::new("Cond[[x > 10 Print[\"Greater than 10\"]] [x < 5 Print[\"Less than 5\"]] [Print[\"Between 5 and 10\"]]]".to_string());
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Cond { conditions, default_statements } => {
                assert_eq!(conditions.len(), 2);
                
                // Check first condition
                match &conditions[0] {
                    (condition, statements) => {
                        match condition {
                            Expression::BinaryOp { left, operator: _, right } => {
                                match **left {
                                    Expression::Identifier(ref name) => assert_eq!(name, "x"),
                                    _ => panic!("Expected x identifier"),
                                }
                            }
                            _ => panic!("Expected binary operation"),
                        }
                        
                        match statements {
                            Expression::FunctionCall { function, arguments } => {
                                match **function {
                                    Expression::Identifier(ref name) => assert_eq!(name, "Print"),
                                    _ => panic!("Expected Print function"),
                                }
                                assert_eq!(arguments.len(), 1);
                                match arguments[0] {
                                    Expression::String(ref msg) => assert_eq!(msg, "Greater than 10"),
                                    _ => panic!("Expected string argument"),
                                }
                            }
                            _ => panic!("Expected function call"),
                        }
                    }
                }
                
                // Check second condition
                match &conditions[1] {
                    (condition, statements) => {
                        match condition {
                            Expression::BinaryOp { left, operator: _, right } => {
                                match **left {
                                    Expression::Identifier(ref name) => assert_eq!(name, "x"),
                                    _ => panic!("Expected x identifier"),
                                }
                            }
                            _ => panic!("Expected binary operation"),
                        }
                        
                        match statements {
                            Expression::FunctionCall { function, arguments } => {
                                match **function {
                                    Expression::Identifier(ref name) => assert_eq!(name, "Print"),
                                    _ => panic!("Expected Print function"),
                                }
                                assert_eq!(arguments.len(), 1);
                                match arguments[0] {
                                    Expression::String(ref msg) => assert_eq!(msg, "Less than 5"),
                                    _ => panic!("Expected string argument"),
                                }
                            }
                            _ => panic!("Expected function call"),
                        }
                    }
                }
                
                // Check default statements
                assert!(default_statements.is_some());
                match *default_statements.unwrap() {
                    Expression::FunctionCall { function, arguments } => {
                        match *function {
                            Expression::Identifier(name) => assert_eq!(name, "Print"),
                            _ => panic!("Expected Print function"),
                        }
                        assert_eq!(arguments.len(), 1);
                        match arguments[0] {
                            Expression::String(ref msg) => assert_eq!(msg, "Between 5 and 10"),
                            _ => panic!("Expected string argument"),
                        }
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected Cond expression"),
        }
    }

    #[test]
    fn test_cond_with_numeric_conditions() {
        let mut parser = Parser::new("Cond[[42 Print[\"The answer\"]] [0 Print[\"Zero\"]]]".to_string());
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Cond { conditions, default_statements } => {
                assert_eq!(conditions.len(), 2);
                
                // Check first condition
                match &conditions[0] {
                    (condition, statements) => {
                        match condition {
                            Expression::Number(num) => assert_eq!(*num, 42),
                            _ => panic!("Expected number"),
                        }
                        
                        match statements {
                            Expression::FunctionCall { function, arguments } => {
                                match **function {
                                    Expression::Identifier(ref name) => assert_eq!(name, "Print"),
                                    _ => panic!("Expected Print function"),
                                }
                                assert_eq!(arguments.len(), 1);
                                match arguments[0] {
                                    Expression::String(ref msg) => assert_eq!(msg, "The answer"),
                                    _ => panic!("Expected string argument"),
                                }
                            }
                            _ => panic!("Expected function call"),
                        }
                    }
                }
                
                // Check second condition
                match &conditions[1] {
                    (condition, statements) => {
                        match condition {
                            Expression::Number(num) => assert_eq!(*num, 0),
                            _ => panic!("Expected number"),
                        }
                        
                        match statements {
                            Expression::FunctionCall { function, arguments } => {
                                match **function {
                                    Expression::Identifier(ref name) => assert_eq!(name, "Print"),
                                    _ => panic!("Expected Print function"),
                                }
                                assert_eq!(arguments.len(), 1);
                                match arguments[0] {
                                    Expression::String(ref msg) => assert_eq!(msg, "Zero"),
                                    _ => panic!("Expected string argument"),
                                }
                            }
                            _ => panic!("Expected function call"),
                        }
                    }
                }
                
                assert!(default_statements.is_none());
            }
            _ => panic!("Expected Cond expression"),
        }
    }

    // Tests for Option type parsing
    #[test]
    fn test_some_parsing() {
        let mut parser = Parser::new("Some[42]".to_string());
        let expr = parser.parse_expression().unwrap();

        eprintln!("Parsed expression: {:?}", expr);

        match expr {
            Expression::Some(value) => {
                match *value {
                    Expression::Number(num) => assert_eq!(num, 42),
                    _ => panic!("Expected number value"),
                }
            }
            _ => panic!("Expected Some expression, got: {:?}", expr),
        }
    }

    #[test]
    fn test_none_parsing() {
        let mut parser = Parser::new("None".to_string());
        let expr = parser.parse_expression().unwrap();

        match expr {
            Expression::None => {},
            _ => panic!("Expected None expression"),
        }
    }

    #[test]
    fn test_some_with_string() {
        let mut parser = Parser::new("Some[\"hello\"]".to_string());
        let expr = parser.parse_expression().unwrap();

        match expr {
            Expression::Some(value) => {
                match *value {
                    Expression::String(s) => assert_eq!(s, "hello"),
                    _ => panic!("Expected string value"),
                }
            }
            _ => panic!("Expected Some expression"),
        }
    }

    // Tests for Result type parsing
    #[test]
    fn test_ok_parsing() {
        let mut parser = Parser::new("Ok[100]".to_string());
        let expr = parser.parse_expression().unwrap();

        match expr {
            Expression::Ok(value) => {
                match *value {
                    Expression::Number(num) => assert_eq!(num, 100),
                    _ => panic!("Expected number value"),
                }
            }
            _ => panic!("Expected Ok expression"),
        }
    }

    #[test]
    fn test_err_parsing() {
        let mut parser = Parser::new("Err[\"error message\"]".to_string());
        let expr = parser.parse_expression().unwrap();

        match expr {
            Expression::Err(error) => {
                match *error {
                    Expression::String(s) => assert_eq!(s, "error message"),
                    _ => panic!("Expected string error"),
                }
            }
            _ => panic!("Expected Err expression"),
        }
    }

    #[test]
    fn test_ok_with_string() {
        let mut parser = Parser::new("Ok[\"success\"]".to_string());
        let expr = parser.parse_expression().unwrap();

        match expr {
            Expression::Ok(value) => {
                match *value {
                    Expression::String(s) => assert_eq!(s, "success"),
                    _ => panic!("Expected string value"),
                }
            }
            _ => panic!("Expected Ok expression"),
        }
    }

    #[test]
    fn test_err_with_number() {
        let mut parser = Parser::new("Err[404]".to_string());
        let expr = parser.parse_expression().unwrap();

        match expr {
            Expression::Err(error) => {
                match *error {
                    Expression::Number(num) => assert_eq!(num, 404),
                    _ => panic!("Expected number error"),
                }
            }
            _ => panic!("Expected Err expression"),
        }
    }
}
