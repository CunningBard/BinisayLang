extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::BinLangParse;
use arg_reader::{ArgMap, ArgReader};
use std::fs;

mod ast;
mod parser;
mod translation;

fn main() {
    let args: ArgMap = ArgReader::new()
        .register("file")
        .bind_with_required(vec!["f", "file"])
        .register("debug")
        .bind(vec!["d", "debug"])
        .register("help")
        .bind(vec!["h", "help"])
        .register("output")
        .bind_with_required(vec!["o", "output"])
        .bind_positional("file")
        .bind_positional("output")
        .read_args(std::env::args().skip(1).collect::<Vec<String>>())
        .unwrap();

    let input_file_name = args.get_as_string("file").unwrap_or_else(|| {
        eprintln!("No file specified, use -f or --file");
        std::process::exit(1);
    });

    let file_data = match fs::read_to_string(input_file_name.clone()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let (data, funcs) = BinLangParse::data(&*file_data);

    let program = translation::BinLangTranslationUnit::translate(data, funcs);

    let output = args.get_as_string("output").unwrap_or_else(|| {
        let file_name = input_file_name.split('.').next().unwrap();
        format!("{}.blc", file_name)
    });

    let mut file = fs::File::create(output).unwrap();
    bincode::serialize_into(&mut file, &program).unwrap();

    if args.flag_is_set("debug") {
        for (line, instruction) in program.instructions.iter().enumerate() {
            println!("{}.\t{:?}", line + 1, instruction);
        }
    }
}
