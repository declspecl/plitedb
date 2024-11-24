use plitedb::{error::PliteDbResult, query::lexer::tokenize};

fn main() -> PliteDbResult<()> {
    let mut input = String::new();

    loop {
        input.clear();

        println!("\nInput query (q to quit):");
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input == "q" || input == "quit" {
            break;
        }

        let tokens = match tokenize(&input) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        println!("{:#?}", tokens);
    }

    return Ok(());
}
