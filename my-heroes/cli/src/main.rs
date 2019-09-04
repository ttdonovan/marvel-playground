use clap::{App, SubCommand};
use exitfailure::ExitFailure;

use my_heroes_backend::db::{
    establish_connection,
    operations::create_hero,
};

fn main() -> Result<(), ExitFailure> {
    let matches = App::new("My Heroes CLI")
        .version("1.0")
        .author("Tanner Donovan <ttdonovan@gmail.com>")
        .about("A CLI sidekick for My Heroes.")
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new hero")
                .args_from_usage("<NAME> 'Creates a Hero from a provided name'")
        ).get_matches();

    match matches.subcommand() {
        ("new", Some(new_matches)) => {
            let name = new_matches.value_of("NAME").unwrap();
            let conn = establish_connection();
            let hero = create_hero(&conn, name)?;
            println!("{:#?}", hero);
        },
        _ => unreachable!(),
    }

    Ok(())
}
