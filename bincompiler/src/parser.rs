use crate::ast::{Expression, Identifier, Statement};
use pest::iterators::Pair;
use pest::Parser;

macro_rules! binary {
    ($left:expr, $right:expr, $op:ident) => {
        Expression::$op {
            left: Box::new($left),
            right: Box::new($right),
        }
    };
}

#[derive(Parser)]
#[grammar = "bin_grammar.pest"]
struct BinLangParser;
pub struct BinLangParse;

impl BinLangParse {
    pub fn identifier_with_dots(pair: Pair<Rule>) -> Identifier {
        let mut pairs = pair.into_inner();
        let mut identifiers = vec![];
        while let Some(pair) = pairs.next() {
            identifiers.push(pair.as_str().to_string());
        }

        Identifier::DotIdentifier(identifiers)
    }

    pub fn identifier(pair: Pair<Rule>) -> Identifier {
        Identifier::Single(pair.as_str().to_string())
    }
    pub fn usable_identifier(pair: Pair<Rule>) -> Identifier {
        let mut pairs = pair.into_inner();
        match pairs.next() {
            Some(pair) => match pair.as_rule() {
                Rule::identifier => Identifier::Single(pair.as_str().to_string()),
                Rule::identifier_with_dots => Self::identifier_with_dots(pair),
                _ => unimplemented!(),
            },
            None => {
                unreachable!()
            }
        }
    }
    pub fn function_call_expr(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        let func_name = Self::identifier(pairs.next().unwrap());
        let mut args = vec![];

        while let Some(pair) = pairs.next() {
            args.push(Self::expr(pair));
        }

        Expression::FunctionCall { func_name, args }
    }
    pub fn term(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        // println!("{:?}", pairs);

        let current = pairs.next().unwrap();

        let left = match current.as_rule() {
            Rule::string => Expression::String(
                current.as_str().to_string()[1..current.as_str().len() - 1].to_string(),
            ),
            Rule::float => Expression::Float(current.as_str().parse().unwrap()),
            Rule::integer => Expression::Int(current.as_str().replace("_", "").parse().unwrap()),
            Rule::bool => Expression::Bool(current.as_str().parse().unwrap()),
            Rule::usable_identifier => Expression::Variable(Self::usable_identifier(current)),
            Rule::function_call => Self::function_call_expr(current),
            Rule::expr => Self::expr(current),
            // Rule::identifier => {
            //     // weird edge case, this should not be a possible rule in this context
            //     // it should be under usable_identifier
            //     // FIXED: it was a bug in the parsing in the conditionals
            //     Expression::Variable(Self::identifier(current))
            // },
            expr_rule => unimplemented!("{:?}", expr_rule),
        };

        left
    }
    pub fn product(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        let current = pairs.next().unwrap();

        let mut left = Self::term(current);

        while let Some(pair) = pairs.next() {
            match pair.as_str() {
                "*" => {
                    left = binary!(left, Self::term(pairs.next().unwrap()), Multiplication);
                }
                "/" => {
                    left = binary!(left, Self::term(pairs.next().unwrap()), Division);
                }
                _ => unimplemented!(),
            }
        }

        left
    }

    pub fn sum(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        let current = pairs.next().unwrap();

        let mut left = Self::product(current);

        while let Some(pair) = pairs.next() {
            match pair.as_str() {
                "+" => {
                    left = binary!(left, Self::product(pairs.next().unwrap()), Addition);
                }
                "-" => {
                    left = binary!(left, Self::product(pairs.next().unwrap()), Subtraction);
                }
                _ => unimplemented!(),
            }
        }

        left
    }
    pub fn expr(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        // println!("{:?}", pairs);

        let current = pairs.next().unwrap();

        let mut left = Self::sum(current);

        while let Some(pair) = pairs.next() {
            match pair.as_str() {
                "==" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), Equal);
                }
                "!=" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), NotEqual);
                }
                "<" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), LessThan);
                }
                ">" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), GreaterThan);
                }
                "<=" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), LessThanOrEqual);
                }
                ">=" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), GreaterThanOrEqual);
                }
                _ => unimplemented!(),
            }
        }

        left
    }

    pub fn variable_assignment(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let identifier = Self::identifier(pairs.next().unwrap());
        let expression = Self::expr(pairs.next().unwrap());

        Statement::Assignment {
            identifier,
            expression,
        }
    }
    pub fn variable_reassignment(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let identifier = Self::identifier(pairs.next().unwrap());
        let expression = Self::expr(pairs.next().unwrap());

        Statement::Reassignment {
            identifier,
            expression,
        }
    }

    pub fn function_call(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let func_name = Self::identifier(pairs.next().unwrap());
        let mut args = vec![];

        while let Some(pair) = pairs.next() {
            args.push(Self::expr(pair));
        }

        Statement::FunctionCall { func_name, args }
    }

    pub fn return_statement(pair: Pair<Rule>) -> Statement {
        let expression = Self::expr(pair.into_inner().next().unwrap());
        Statement::Return(expression)
    }

    pub fn conditional(pair: Pair<Rule>, in_a_function: bool) -> Statement {
        let mut pairs = pair.into_inner();

        let mut current = pairs.next().unwrap();
        let mut conditions = vec![];

        while current.as_rule() == Rule::expr {
            let condition = Self::expr(current);
            let body = Self::block(pairs.next().unwrap(), true, in_a_function);

            conditions.push((condition, body));

            match pairs.next() {
                Some(pair) => {
                    current = pair;
                }
                None => {
                    return Statement::Conditional {
                        body: conditions,
                        else_body: None,
                    }
                }
            }
        }

        let else_body = if current.as_rule() == Rule::block {
            Some(Self::block(current, true, in_a_function))
        } else {
            None
        };

        Statement::Conditional {
            body: conditions,
            else_body,
        }
    }

    pub fn while_loop(pair: Pair<Rule>, in_a_function: bool) -> Statement {
        let mut pairs = pair.into_inner();
        let condition = Self::expr(pairs.next().unwrap());
        let body = Self::block(pairs.next().unwrap(), true, in_a_function);

        Statement::WhileLoop { condition, body }
    }

    pub fn function_definition(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let func_name = Self::identifier(pairs.next().unwrap());
        let mut args = vec![];

        let mut current = pairs.next().unwrap();

        while Rule::identifier == current.as_rule() {
            args.push(Self::identifier(current));

            current = pairs.next().unwrap();
        }

        let body = Self::block(current, false, true);

        Statement::FunctionDeclaration {
            func_name,
            args,
            body,
        }
    }

    pub fn block(pair: Pair<Rule>, in_a_loop: bool, in_a_function: bool) -> Vec<Statement> {
        let mut pairs = pair.into_inner();
        let mut statements = vec![];

        while let Some(pair) = pairs.next() {
            statements.push(Self::statement(pair, in_a_loop, in_a_function).expect("Parse Error"));
        }

        statements
    }

    pub fn statement(pair: Pair<Rule>, in_a_loop: bool, in_a_function: bool) -> Option<Statement> {
        let data = match pair.as_rule() {
            Rule::variable_assignment => Self::variable_assignment(pair),
            Rule::variable_reassignment => Self::variable_reassignment(pair),
            Rule::function_call => Self::function_call(pair),
            Rule::return_statement => {
                if !in_a_function {
                    return None;
                };

                Self::return_statement(pair)
            }
            Rule::conditional => Self::conditional(pair, in_a_function),
            Rule::while_loop => Self::while_loop(pair, in_a_function),
            Rule::break_statement => {
                if !in_a_loop {
                    return None;
                };

                Statement::Break
            }
            Rule::continue_statement => {
                if !in_a_loop {
                    return None;
                };

                Statement::Continue
            }
            Rule::function_definition => {
                if in_a_function {
                    return None;
                };

                Self::function_definition(pair)
            }
            Rule::EOI => Statement::EOI,
            Rule::single_line_comment | Rule::multi_line_comment => {
                Statement::Comment(pair.as_str().to_string())
            }
            what => unreachable!("{:?}", what),
        };

        Some(data)
    }

    pub fn data(data: &str) -> (Vec<Statement>, Vec<Statement>) {
        let pairs = match BinLangParser::parse(Rule::program, data) {
            Ok(pairs) => pairs,
            Err(e) => panic!("Error: {}", e),
        };

        // for pair in pairs.clone() {
        //     println!("{:#?}", pair);
        // }

        let mut statements = vec![];
        let mut functions = vec![];

        for pair in pairs.clone() {
            match Self::statement(pair, false, false) {
                Some(Statement::FunctionDeclaration {
                    func_name,
                    args,
                    body,
                }) => {
                    functions.push(Statement::FunctionDeclaration {
                        func_name,
                        args,
                        body,
                    });
                }
                Some(statement) => {
                    statements.push(statement);
                }
                None => {}
            }
        }

        (statements, functions)
    }
}
