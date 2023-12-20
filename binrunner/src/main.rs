use std::rc::Rc;
use bincore;
use bincore::executable::runnable::Instruction;
use bincore::executable::runtime::Runtime;
use bincore::object::Value;


fn value_into_printable(value: Value) -> String {
    match value {
        Value::Int(value) => value.to_string(),
        Value::Float(value) => value.to_string(),
        Value::Str(value) => value,
        Value::Bool(value) => value.to_string(),
        Value::List(value) => value.into_iter().map(|value| value_into_printable(value)).collect::<Vec<String>>().join(", "),
        Value::Object(value) => format!("Object({})", value.descriptor.name)
    }
}

fn println(runtime: &mut Runtime) {
    let len = runtime.stack.pop().unwrap().as_int().unwrap();
    let mut values = Vec::new();

    for _ in 0..len {
        values.push(runtime.stack.pop().unwrap());
    }

    println!("{}",
             values.into_iter()
                 .map(|value| value_into_printable(value))
                 .collect::<Vec<String>>()
                 .join(" ")
    );
}

fn main() {
    let mut runtime = Runtime::new();

    runtime.register_function("println".to_string(), Rc::new(Box::new(println)));

    let instructions = vec![
        Instruction::Push { value: Value::Str("Hardcoded Hello World!".to_string()) },
        Instruction::Push { value: Value::Int(1) },
        Instruction::Call { function: "println".to_string() },
    ];
    runtime.run(instructions);

    // println!("{:?}", runtime.heap);
}