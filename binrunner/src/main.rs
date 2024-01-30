use bincore;
use bincore::data::program_file::Program;
use bincore::data::value::Value;
use bincore::executable::runtime::Runtime;

fn value_into_printable(value: Value, runtime: &mut Runtime) -> String {
    match value {
        Value::Int(value) => value.to_string(),
        Value::Float(value) => value.to_string(),
        Value::StrRef(value) => runtime.strings[value].clone(),
        Value::Bool(value) => value.to_string(),
        Value::ListRef(value) => {
            let list = runtime.lists.get(&value).unwrap().clone();
            let mut stringed = vec![];

            for value in list {
                stringed.push(value_into_printable(value.clone(), runtime));
            }

            format!("[{}]", stringed.join(", "))
        }
        Value::ObjectRef(value) => {
            let object = runtime.objects.get(&value).unwrap().clone();

            let descriptor = object.descriptor.clone();

            let mut string = vec![];

            for (name, index) in descriptor.members.iter() {
                let value = object.members.get(*index).unwrap();
                string.push(format!(
                    "{}: {}",
                    name,
                    value_into_printable(value.clone(), runtime)
                ));
            }

            format!("{} {{ {} }}", descriptor.name, string.join(", "))
        }
        Value::Char(value) => value.to_string(),
    }
}

fn println(runtime: &mut Runtime) {
    let len = runtime.stack_pop().as_int().unwrap();
    let mut values = Vec::new();

    for _ in 0..len {
        values.push(runtime.stack_pop());
    }

    println!(
        "{}",
        values
            .into_iter()
            .map(|value| value_into_printable(value, runtime))
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn push(runtime: &mut Runtime) {
    let list = runtime.stack_pop();
    let value = runtime.stack_pop();

    let list = runtime.lists.get_mut(&list.as_list_ref().unwrap()).unwrap();
    list.push(value);
}

fn pop(runtime: &mut Runtime) {
    let list = runtime.stack_pop();

    let list = runtime.lists.get_mut(&list.as_list_ref().unwrap()).unwrap();
    let value = list.pop().unwrap();
    runtime.stack_push(value);
}

fn new_list(runtime: &mut Runtime) {
    let list = runtime.list_init_counter;
    runtime.list_init_counter += 1;

    runtime.lists.insert(list, Vec::new());
    runtime.stack_push(Value::ListRef(list));
}

fn new_list_with_values(runtime: &mut Runtime) {
    let list = runtime.list_init_counter;
    runtime.list_init_counter += 1;

    let mut values = Vec::new();
    let len = runtime.stack_pop().as_int().unwrap();

    for _ in 0..len {
        values.push(runtime.stack_pop());
    }

    runtime.lists.insert(list, values);
    runtime.stack_push(Value::ListRef(list));
}

fn new_list_with_default_values(runtime: &mut Runtime) {
    let list = runtime.list_init_counter;
    runtime.list_init_counter += 1;

    let len = runtime.stack_pop().as_int().unwrap();
    let value = runtime.stack_pop();

    let mut values = Vec::new();

    for _ in 0..len {
        values.push(value.clone());
    }

    runtime.lists.insert(list, values);
    runtime.stack_push(Value::ListRef(list));
}

fn index_set(runtime: &mut Runtime) {
    let obj = runtime.stack_pop().as_list_ref().unwrap();
    let index = runtime.stack_pop().as_int().unwrap();
    let value = runtime.stack_pop();

    let list = runtime.lists.get_mut(&obj).unwrap();
    list[index as usize] = value;
}

fn index_get(runtime: &mut Runtime) {
    let list = runtime.stack_pop();
    let index = runtime.stack_pop().as_int().unwrap();

    match list {
        Value::ListRef(list) => {
            let list = runtime.lists.get(&list).unwrap();
            runtime.stack_push(list[index as usize].clone());
        }
        Value::StrRef(string_id) => {
            let char = runtime
                .string_objects
                .get(&string_id)
                .unwrap()
                .chars()
                .nth(index as usize)
                .unwrap();
            runtime.stack_push(Value::Char(char));
        }
        _ => {
            panic!("Cannot index non-list or non-object");
        }
    }
}

fn len(runtime: &mut Runtime) {
    let value = runtime.stack_pop();

    match value {
        Value::ListRef(list) => {
            let list = runtime.lists.get(&list).unwrap();
            runtime.stack_push(Value::Int(list.len() as i64));
        }
        Value::StrRef(string) => {
            let string = runtime.strings.get(string).unwrap();
            runtime.stack_push(Value::Int(string.len() as i64));
        }
        _ => {
            panic!("Cannot get length of non-list or non-object");
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let _program_name = args[0].clone();
    let mut args_index = 1;
    let mut input = "test.blc".to_string();

    while args_index < args.len() {
        match args[args_index].as_str() {
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

    let input_file = std::fs::read(input).unwrap();
    let program: Program = bincode::deserialize(&input_file).unwrap();

    let mut runtime = program.into_runtime();

    macro_rules! register_function {
        ($func:expr) => {
            runtime.register_function(stringify!($func).to_string(), $func);
        };
    }

    register_function!(println);
    register_function!(push);
    register_function!(pop);
    register_function!(new_list);
    register_function!(new_list_with_values);
    register_function!(new_list_with_default_values);
    register_function!(index_set);
    register_function!(index_get);
    register_function!(len);

    runtime.run()
}
