use bincore;
use bincore::data::program_file::Program;
use bincore::executable::runtime::Runtime;
use bincore::data::value::Value;


fn value_into_printable(value: Value, runtime: &mut Runtime) -> String {
    match value {
        Value::Int(value) => value.to_string(),
        Value::Float(value) => value.to_string(),
        Value::Str(value) => value,
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
                string.push(format!("{}: {}", name, value_into_printable(value.clone(), runtime)));
            }

            format!("{} {{ {} }}", descriptor.name, string.join(", "))
        }
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
                 .map(|value| value_into_printable(value, runtime))
                 .collect::<Vec<String>>()
                 .join(" ")
    );
}


fn push(runtime: &mut Runtime){
    let list = runtime.stack.pop().unwrap();
    let value = runtime.stack.pop().unwrap();

    let list = runtime.lists.get_mut(&list.as_list_ref().unwrap()).unwrap();
    list.push(value);
}

fn pop(runtime: &mut Runtime) {
    let list = runtime.stack.pop().unwrap();

    let list = runtime.lists.get_mut(&list.as_list_ref().unwrap()).unwrap();
    runtime.stack.push(list.pop().unwrap());
}

fn new_list(runtime: &mut Runtime) {
    let list = runtime.list_init_counter;
    runtime.list_init_counter += 1;

    runtime.lists.insert(list, Vec::new());
    runtime.stack.push(Value::ListRef(list));
}

fn new_list_with_values(runtime: &mut Runtime) {
    let list = runtime.list_init_counter;
    runtime.list_init_counter += 1;

    let mut values = Vec::new();
    let len = runtime.stack.pop().unwrap().as_int().unwrap();

    for _ in 0..len {
        values.push(runtime.stack.pop().unwrap());
    }

    runtime.lists.insert(list, values);
    runtime.stack.push(Value::ListRef(list));
}

fn new_list_with_default_values(runtime: &mut Runtime) {
    let list = runtime.list_init_counter;
    runtime.list_init_counter += 1;

    let len = runtime.stack.pop().unwrap().as_int().unwrap();
    let value = runtime.stack.pop().unwrap();

    let mut values = Vec::new();

    for _ in 0..len {
        values.push(value.clone());
    }

    runtime.lists.insert(list, values);
    runtime.stack.push(Value::ListRef(list));
}

fn index_set(runtime: &mut Runtime) {
    let obj = runtime.stack.pop().unwrap().as_list_ref().unwrap();
    let index = runtime.stack.pop().unwrap().as_int().unwrap();
    let value = runtime.stack.pop().unwrap();

    let list = runtime.lists.get_mut(&obj).unwrap();
    list[index as usize] = value;
}

fn index_get(runtime: &mut Runtime) {
    let list = runtime.stack.pop().unwrap();
    let index = runtime.stack.pop().unwrap().as_int().unwrap();

    match list {
        Value::ListRef(list) => {
            let list = runtime.lists.get(&list).unwrap();
            runtime.stack.push(list[index as usize].clone());
        }
        Value::Str(string) => {
            let char = string.chars().nth(index as usize).unwrap();
            runtime.stack.push(Value::Str(char.to_string()));
        }
        _ => {
            panic!("Cannot index non-list or non-object");
        }

    }
}

fn len(runtime: &mut Runtime) {
    let value = runtime.stack.pop().unwrap();

    match value {
        Value::ListRef(list) => {
            let list = runtime.lists.get(&list).unwrap();
            runtime.stack.push(Value::Int(list.len() as i64));
        }
        Value::Str(string) => {
            runtime.stack.push(Value::Int(string.len() as i64));
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

    let mut runtime: Runtime = Runtime::new();

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

    for object_builder in program.object_builder.clone() {
        let name = object_builder.name.clone();
        let object_builder = object_builder.into_builder();
        runtime.object_builders.insert(name, object_builder);
    }

    runtime.run(program.instructions)
}