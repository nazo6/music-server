use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Library {
    pub name: String,
    pub id: i32,
    pub path: String,
}

#[derive(SimpleObject)]
pub struct User {
    pub name: String,
    pub is_admin: bool,
}

#[derive(SimpleObject)]
pub struct ScanStatus {
    pub scanning: bool,
    pub proceed_count: i32,
}
