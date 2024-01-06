extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::BinLangParse;

mod parser;
mod ast;
mod translation;

fn main() {
    let file = "./test.bsl";
    let file_data = std::fs::read_to_string(file).unwrap();

    let (data, funcs) = BinLangParse::data(&*file_data);

    println!("\t ==================== \t");

    for (line, statement) in data.iter().enumerate() {
        println!("{}.\t{:?}", line + 1, statement);
    }

    println!("\t ==================== \t");

    let program = translation::BinLangTranslationUnit::translate(data, funcs);

    for (line, instruction) in program.instructions.iter().enumerate() {
        println!("{}.\t{:?}", line + 1, instruction);
    }


    let mut file = std::fs::File::create("./test.blc").unwrap();
    bincode::serialize_into(&mut file, &program).unwrap();
}
