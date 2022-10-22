use diesel::prelude::*;
use crate::schema::analytics;

#[derive(Queryable)]
pub struct Analytic {
    pub id: i32,
}


#[derive(Insertable)]
#[diesel(table_name = analytics)]
pub struct NewAnalytic<'a> {
    pub user_agent: &'a str,
}

