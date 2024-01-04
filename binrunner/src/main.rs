use bincore;
use bincore::data::function::{Callable, Function};
use bincore::data::program_file::Program;
use bincore::executable::runnable::Instruction;
use bincore::executable::runtime::Runtime;
use bincore::data::value::Value;


fn value_into_printable(value: Value) -> String {
    match value {
        Value::Int(value) => value.to_string(),
        Value::Float(value) => value.to_string(),
        Value::Str(value) => value,
        Value::Bool(value) => value.to_string(),
        Value::List(value) => value.into_iter().map(|value| value_into_printable(value)).collect::<Vec<String>>().join(", "),
        Value::ObjectRef(value) => format!("Object({:#08x})", value)
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
    let args = std::env::args().collect::<Vec<String>>();
    let _program_name = args[0].clone();
    let mut args_index = 1;

    let mut debug_mode = false;
    let mut input = "main.blc".to_string();

    while args_index < args.len() {
        match args[args_index].as_str() {
            "-d" => {
                debug_mode = true;
            }
            "-i" => {
                input = args[args_index + 1].clone();
                args_index += 1;
            }
            option => {
                panic!("Unknown option {}", option)
            }
        }

        args_index += 1;
    }

    let input_file = std::fs::read_to_string(input).unwrap();
    let program = bincode::deserialize::<Program>(&input_file.as_bytes()).unwrap();

    let mut runtime = Runtime::new();

    for func in program.functions.clone() {
        runtime.register_function(func.name.clone(), Callable::Function(func));
    }

    runtime.run(program.instructions)
}