use std::collections::HashMap;
use bincore::data::program_file::Program;
use bincore::data::value::Value;
use bincore::executable::runnable::Instruction;
use crate::ast::{Expression, Statement};


const VARIADIC_FUNCTIONS: [&str; 1] = [
    "println"
];

const EXTERNAL_FUNCTIONS: [&str; 1] = [
    "println"
];

pub enum IntermediateCode {
    Label(String),
    Call(String),
    Inst(Instruction),
    Jump(String),
    JumpIfTrue(String),
    JumpIfFalse(String),
}

pub struct IC;

impl IC {
    pub fn call(name: &str) -> IntermediateCode {
        IntermediateCode::Call(name.to_string())
    }
    pub fn label(name: &str) -> IntermediateCode {
        IntermediateCode::Label(name.to_string())
    }
    pub fn instruction(instruction: Instruction) -> IntermediateCode {
        IntermediateCode::Inst(instruction)
    }
    pub fn jump(name: &str) -> IntermediateCode {
        IntermediateCode::Jump(name.to_string())
    }
    pub fn jump_if_true(name: &str) -> IntermediateCode {
        IntermediateCode::JumpIfTrue(name.to_string())
    }
    pub fn jump_if_false(name: &str) -> IntermediateCode {
        IntermediateCode::JumpIfFalse(name.to_string())
    }
}

pub struct BinLangTranslationUnit {
    pub statements: Vec<Statement>,
    pub functions: Vec<Statement>,
    pub code: Vec<IntermediateCode>,

    pub func_args: HashMap<String, Vec<String>>,

    conditional_label_count: usize,
    while_label_count: usize,
}

impl BinLangTranslationUnit {

    pub fn expression(&mut self, expression: &Expression) -> Vec<IntermediateCode> {
        let mut code = vec![];

        macro_rules! operation {
            ($left:expr, $right:expr, $op:ident) => {
                code.append(&mut self.expression($left));
                code.append(&mut self.expression($right));
                code.push(IC::instruction(Instruction::$op));
            }
        }

        match expression {
            Expression::Int(value) => {
                code.push(IC::instruction(Instruction::Push {
                    value: Value::Int(*value)
                }));
            }
            Expression::Float(value) => {
                code.push(IC::instruction(Instruction::Push {
                    value: Value::Float(*value)
                }));
            }
            Expression::String(value) => {
                code.push(IC::instruction(Instruction::Push {
                    value: Value::Str(value.to_string())
                }));
            }
            Expression::Bool(value) => {
                code.push(IC::instruction(Instruction::Push {
                    value: Value::Bool(*value)
                }));
            }
            Expression::Variable(variable) => {
                code.push(IC::instruction(Instruction::Load {
                    name: variable.to_string()
                }));
            }
            Expression::FunctionCall { func_name, args } => {
                code.append(&mut self.function_call(&*func_name.to_string(), args));
            }
            Expression::Addition { left, right } => {
                operation!(left, right, Add);
            }
            Expression::Subtraction { left, right } => {
                operation!(left, right, Sub);
            }
            Expression::Multiplication { left, right } => {
                operation!(left, right, Mul);
            }
            Expression::Division { left, right } => {
                operation!(left, right, Div);
            }
            Expression::Equal { left, right } => {
                operation!(left, right, Eq);
            }
            Expression::NotEqual { left, right } => {
                operation!(left, right, Neq);
            }
            Expression::GreaterThan { left, right } => {
                operation!(left, right, Gt);
            }
            Expression::LessThan { left, right } => {
                operation!(left, right, Lt);
            }
            Expression::GreaterThanOrEqual { left, right } => {
                operation!(left, right, Gte);
            }
            Expression::LessThanOrEqual { left, right } => {
                operation!(left, right, Lte);
            }
        }

        code
    }

    pub fn assignment(&mut self, identifier: &str, expression: &Expression) -> Vec<IntermediateCode> {
        let mut code = vec![];

        code.append(&mut self.expression(expression));
        code.push(IC::instruction(Instruction::Store {
            name: identifier.to_string()
        }));

        code
    }
    pub fn function_call(&mut self, func_name: &str, args: &Vec<Expression>) -> Vec<IntermediateCode> {
        let mut code = vec![];


        for arg in args.iter().rev() {
            code.append(&mut self.expression(arg));
        }

        if EXTERNAL_FUNCTIONS.contains(&func_name) {
            if VARIADIC_FUNCTIONS.contains(&func_name) {
                code.push(IC::instruction(Instruction::Push {
                    value: Value::Int(args.len() as i64)
                }));
            }

            code.push(IC::instruction(Instruction::ExternCall {
                function: func_name.to_string()
            }));
        } else {
            for arg_name in self.func_args.get(func_name).unwrap().iter().rev() {
                code.push(IC::instruction(Instruction::Store {
                    name: arg_name.to_string()
                }));
            }
            code.push(IC::call(format!("function_{}", func_name).as_str()));
        }

        code
    }

    pub fn return_statement(&mut self, expression: &Expression) -> Vec<IntermediateCode> {
        let mut code = vec![];

        code.append(&mut self.expression(expression));
        code.push(IC::instruction(Instruction::Ret));

        code
    }

    pub fn conditional(
        &mut self,
        bodies: Vec<(Expression, Vec<Statement>)>,
        else_body: Option<Vec<Statement>>
    ) -> Vec<IntermediateCode>{
        let mut intermediate = vec![];

        let mut if_counts = 0;
        let condition_counts = self.conditional_label_count;

        let end_label = format!("end_{}", self.conditional_label_count);
        self.conditional_label_count += 1;

        for (condition, body) in bodies {
            let elif_label = format!("if_{}_{}", condition_counts, if_counts);
            if_counts += 1;

            intermediate.append(&mut self.expression(&condition));
            intermediate.push(IC::jump_if_false(&elif_label));
            for statement in body {
                intermediate.append(&mut self.statement(&statement));
            }
            intermediate.push(IC::jump(&end_label));
            intermediate.push(IC::label(&elif_label));
        }

        if let Some(body) = else_body {
            for statement in body {
                intermediate.append(&mut self.statement(&statement));
            }
        } else {
            let label = intermediate.pop().unwrap();
            intermediate.pop().unwrap();

            intermediate.push(label);
        }

        intermediate.push(IC::label(&end_label));
        intermediate
    }
    
    pub fn while_loop(&mut self, condition: &Expression, body: &Vec<Statement>) -> Vec<IntermediateCode> {
        let mut intermediate = vec![];

        let condition_label = format!("while_{}", self.while_label_count);
        let end_label = format!("end_while_{}", self.while_label_count);
        self.while_label_count += 1;

        intermediate.push(IC::label(&condition_label));
        intermediate.append(&mut self.expression(condition));
        intermediate.push(IC::jump_if_false(&end_label));

        for statement in body {
            intermediate.append(&mut self.statement(statement));
        }

        intermediate.push(IC::jump(&condition_label));
        intermediate.push(IC::label(&end_label));

        intermediate
    }
    pub fn statement(&mut self, statement: &Statement) -> Vec<IntermediateCode> {
        let mut intermediate = vec![];
        match statement {
            Statement::Assignment { identifier, expression } => {
                intermediate.append(&mut self.assignment(&*identifier.to_string(), expression))
            }
            Statement::Reassignment { identifier, expression } => {
                intermediate.append(&mut self.assignment(&*identifier.to_string(), expression))
            }
            Statement::FunctionCall { func_name, args } => {
                intermediate.append(&mut self.function_call(&*func_name.to_string(), args))
            }
            Statement::FunctionDeclaration { func_name, args, body } => {
                intermediate.append(&mut self.function_declaration(
                    &*func_name.to_string(),
                    args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().as_ref(),
                    body
                ))
            }
            Statement::Conditional { body, else_body } => {
                intermediate.append(&mut self.conditional(body.clone(), else_body.clone()))
            }
            Statement::WhileLoop { condition, body } => {
                intermediate.append(&mut self.while_loop(condition, body))
            }
            Statement::Break => { unimplemented!() }
            Statement::Continue => { unimplemented!()}
            Statement::EOI => {}
            Statement::Return(expression) => {
                intermediate.append(&mut self.return_statement(expression))
            }
            Statement::Comment(_) => {}
        }

        intermediate
    }

    pub fn function_declaration(&mut self, func_name: &str, args: &Vec<String>, body: &Vec<Statement>) -> Vec<IntermediateCode>{
        let mut intermediate = vec![];

        self.func_args.insert(func_name.to_string(), args.clone());

        intermediate.push(IC::label(format!("function_{}", func_name).as_str()));

        for statement in body {
            intermediate.append(&mut self.statement(statement));
        }

        intermediate
    }

    pub fn run(&mut self) -> Vec<Instruction> {
        let mut intermediate = vec![
            IC::jump("_start"),
        ];

        for code in self.functions.clone().iter() {
            intermediate.extend(self.statement(code))
        }

        intermediate.push(IC::label("_start"));

        for code in self.statements.clone().iter() {
            intermediate.extend(self.statement(code))
        }

        intermediate.push(IC::instruction(Instruction::Nop));


        let mut labels = HashMap::new();
        let mut counter = 1;

        let mut new_intermediate = vec![];

        let mut code = vec![];

        for instruction in intermediate {
            match instruction {
                IntermediateCode::Label(name) => {
                    labels.insert(name, counter);
                }
                _ => {
                    counter += 1;
                    new_intermediate.push(instruction);
                }
            }
        }

        let get_label = |name: &str| {
            *labels.get(&*name.to_string()).unwrap()
        };

        for instruction in new_intermediate {
            match instruction {
                IntermediateCode::Label(_) => {}
                IntermediateCode::Inst(inst) => {
                    code.push(inst);
                }
                IntermediateCode::Jump(name) => {
                    code.push(Instruction::Jump {
                        address: get_label(&*name)
                    });
                }
                IntermediateCode::JumpIfTrue(name) => {
                    code.push(Instruction::JumpIfTrue {
                        address: get_label(&*name)
                    });
                }
                IntermediateCode::JumpIfFalse(name) => {
                    code.push(Instruction::JumpIfFalse {
                        address: get_label(&*name)
                    });
                }
                IntermediateCode::Call(name) => {
                    code.push(Instruction::Call {
                        address: get_label(&*name)
                    });
                }
            }
        }

        code
    }

    pub fn translate(statements: Vec<Statement>, functions: Vec<Statement>) -> Program {
        let mut unit = BinLangTranslationUnit {
            statements,
            functions,
            code: vec![],
            func_args: Default::default(),
            conditional_label_count: 0,
            while_label_count: 0,
        };

        let inst= unit.run();

        Program {
            instructions: inst,
            object_builder: vec![],
        }
    }
}