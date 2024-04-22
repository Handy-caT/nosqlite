mod api;
mod command;
mod r#static;

use crate::{api::Api, r#static::welcome};
use backend_api::api::command::{r#enum::BackendCommand, Gateway};
use frontend::planner::adapter::PlannerCommand;
use std::{io, io::Write};
use crate::command::execute_frontend_command;

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn repl<const NODE_SIZE: u8>(mut api: Api<NODE_SIZE>) -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buffer)?;

        println!("{}", buffer);
        api.frontend_api.set_input(&buffer);
        let commands = api.frontend_api.commands();

        for command in commands {
            match command {
                Ok(command) => {
                    println!("{:?}", command);

                    match command {
                        PlannerCommand::Backend(command) => match command {
                            BackendCommand::Database(command) => {
                                if let Err(error) =
                                    api.backend_api.send(command)
                                {
                                    println!("{:?}", error);
                                }
                            }
                            BackendCommand::Schema(command) => {
                                if let Err(error) =
                                    api.backend_api.send(command)
                                {
                                    println!("{:?}", error);
                                }
                            }
                        },
                        PlannerCommand::Frontend(command) => {
                            if let Err(error) = execute_frontend_command(&mut api, command)
                            {
                                println!("{:?}", error);
                            }
                        }
                    }
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        }
        
        if api.context.quit {
            return Ok(());
        }
        
        buffer.clear();
    }
}

fn main() -> io::Result<()> {
    let api = Api::<128>::default();

    clear_screen();
    welcome();
    repl(api)?;

    Ok(())
}
