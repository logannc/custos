use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct BeatRecord {
    pub id: i32,
    pub quantity: i32,
    pub is_arcane: bool,
    pub player: String,
}