use clap::App;
use clap::Arg;

struct Program {
    action: String,
    option: String,
}

fn load_program(action:String, option:String) -> Program {
    Program {
        action: action,
        option: option,
    }
}

fn main() {
    println!("Este es un cli echo en rust para practicar mis conocimientos");

    let arg1 = Arg::with_name("action")
        .short("a")
        .value_name("action")
        .help("Permite definir la accion a realizar")
        .required(true);

    let arg2 = Arg::with_name("option")
        .short("o")
        .value_name("option")
        .help("Permite definir las opciones")
        .required(true);

    let matches = App::new("CLI")
        .version("0.0.1")
        .about("Simple CLI")
        .arg(arg1)
        .arg(arg2)
        .author("Bryan Graterol <baafs@gmail.com> ")
        .get_matches();

    let resultado: Program = load_program(
        matches.value_of("action").unwrap().to_string(),
        matches.value_of("option").unwrap().to_string(),
    );

    println!(">>> {} - {}", resultado.action, resultado.option);
}
