use diesel::prelude::*;

use crate::schema::cvs;
use crate::models::connections::establish_connection;

#[derive(Queryable)]
pub struct CV {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author: String
}

impl CV {
    pub fn new( new_id: i32, new_title: String, new_body: String, new_author: String) -> Self {
        Self { 
            id: new_id,
            body: new_body,
            title: new_title,
            author: new_author
        }
    }

    pub fn to_CV_db(&self) {}
}

#[derive(Insertable)]
#[diesel(table_name = cvs)]
pub struct NewCV<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub author: &'a str 
}


fn create_cv(new_title: &str, new_author: &str, new_body: &str) -> usize {
    use crate::schema::cvs::dsl::*;

    let inserted_cv = NewCV {
        title: new_title,
        author: new_author,
        body: new_body
    };

    // TODO Change to DATAPOOL
    let connection = &mut establish_connection();

    diesel::insert_into(cvs)
        .values(&inserted_cv)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Can't insert new value: {:?}", e))
}


fn delete_cv(id: String) -> usize {
    use crate::schema::cvs::dsl::*;

    let connection = &mut establish_connection();

    diesel::delete(cvs.filter(id.eq(id)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Can't delete current value {:?}", e))
}
