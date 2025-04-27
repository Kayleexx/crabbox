use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::books;

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published_year: i32,
}