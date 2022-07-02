use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Scene {
    pub id: i32,
    pub name: String,
}