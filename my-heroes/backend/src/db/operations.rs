use diesel::{
    prelude::*,
    result::Error,
    sqlite::SqliteConnection,
};

use super::models;

pub fn create_hero<'a>(conn: &SqliteConnection, hero_name: &'a str) -> Result<models::Hero, Error> {
    use super::schema::heroes::dsl::*;

    let new_hero = models::NewHero { name: hero_name };

    let hero = conn.transaction::<_, Error, _>(|| {
        diesel::insert_into(heroes)
            .values(&new_hero)
            .execute(conn)?;
        
        heroes.order(id.desc()).first(conn)
    });

    hero
}