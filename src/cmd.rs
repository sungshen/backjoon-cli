use crate::cli::Commands;
use crate::web::submit::submit;
use crate::web::login::login;

pub(crate) fn option(chosen: &Option<Commands>) {
    match chosen {
        Some(x) => match x {
            Commands::T => compile(),
            Commands::S => submit(),
            Commands::L { id, password } => login(Some(id), Some(password)),
            Commands::Q => login(None, None),
            },
        None => println!("설명"),
    }
}

fn compile() {
}
