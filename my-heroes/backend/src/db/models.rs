use super::schema::heroes;

#[derive(Debug, Queryable)]
pub struct Hero {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "heroes"]
pub struct NewHero<'a> {
    pub name: &'a str,
}