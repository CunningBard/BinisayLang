use arg_reader::ArgReader;
use bincore;
use bincore::data::program_file::Program;
use bincore::data::value::Value;
use bincore::executable::runtime::Runtime;

fn value_into_printable(value: Value, runtime: &mut Runtime) -> String {
    match value {
        Value::Int(value) => value.to_string(),
        Value::Float(value) => value.to_string(),
        Value::StrRef(value) => runtime.string_objects[&value].clone(),
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

fn ipakita(runtime: &mut Runtime) {
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

fn butngan(runtime: &mut Runtime) {
    let list = runtime.stack_pop();
    let value = runtime.stack_pop();

    let list = runtime.lists.get_mut(&list.as_list_ref().unwrap()).unwrap();
    list.push(value);
}

fn kuhaan(runtime: &mut Runtime) {
    let list = runtime.stack_pop();

    let list = runtime.lists.get_mut(&list.as_list_ref().unwrap()).unwrap();
    let value = list.pop().unwrap();
    runtime.stack_push(value);
}

fn bag_ong_lista(runtime: &mut Runtime) {
    let list = runtime.list_init_counter;
    runtime.list_init_counter += 1;

    runtime.lists.insert(list, Vec::new());
    runtime.stack_push(Value::ListRef(list));
}

fn bag_ong_list_nga_naay_sulod(runtime: &mut Runtime) {
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

fn index_set(runtime: &mut Runtime) {
    let obj = runtime.stack_pop().as_list_ref().unwrap();
    let index = runtime.stack_pop().as_int().unwrap();
    let value = runtime.stack_pop();

    let list = runtime.lists.get_mut(&obj).unwrap();
    list[index as usize] = value;
}

fn indeks_kuha(runtime: &mut Runtime) {
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

fn katas_on(runtime: &mut Runtime) {
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
    let args = ArgReader::new()
        .register("file_path")
        .bind_with_required(vec!["i", "input"])
        .register("debug")
        .bind(vec!["d", "debug"])
        .bind_positional("file_path")
        .read_args(std::env::args().skip(1).collect())
        .unwrap();

    let file_path = match args.get_as_string("file_path") {
        Some(value) => value,
        None => panic!("No file path provided"),
    };
    let input_file = std::fs::read(file_path).unwrap();
    let program: Program = bincode::deserialize(&input_file).unwrap();

    let mut runtime = program.into_runtime();

    macro_rules! register_function {
        ($func:expr) => {
            runtime.register_function(stringify!($func).to_string(), $func);
        };
    }

    register_function!(ipakita);
    register_function!(butngan);
    register_function!(kuhaan);
    register_function!(bag_ong_lista);
    register_function!(bag_ong_list_nga_naay_sulod);
    register_function!(index_set);
    register_function!(indeks_kuha);
    register_function!(katas_on);

    runtime.run()
}
