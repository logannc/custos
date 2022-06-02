use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Channel {
    pub id: i32,
    pub discord_id: i64,
    pub name: String,
}
