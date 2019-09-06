use clap::{App, SubCommand};
use exitfailure::ExitFailure;

use my_heroes_backend::db::{
    establish_connection,
    operations::{
        create_hero,
        find_hero,
        query_heroes,
    },
};

fn main() -> Result<(), ExitFailure> {
    let matches = App::new("My Heroes CLI")
        .version("1.0")
        .author("Tanner Donovan <ttdonovan@gmail.com>")
        .about("A CLI sidekick for My Heroes.")
        .subcommand(
            SubCommand::with_name("find")
                .about("Find a hero")
                .args_from_usage("<NAME> 'Finds a Hero from a provided name'")
        ).subcommand(
            SubCommand::with_name("list")
                .about("Lists all heroes")
        ).subcommand(
            SubCommand::with_name("new")
                .about("Creates a new hero")
                .args_from_usage("<NAME> 'Creates a Hero from a provided name'")
        ).get_matches();

    match matches.subcommand() {
        ("find", Some(find_matches)) => {
            let name = find_matches.value_of("NAME").unwrap();
            let conn = establish_connection();
            let hero = find_hero(&conn, name)?;
            println!("{:#?}", hero);
        },
        ("list", _) => {
            let conn = establish_connection();
            let heros = query_heroes(&conn)?;
            println!("{:#?}", heros);
        },
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
