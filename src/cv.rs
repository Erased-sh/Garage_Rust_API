use diesel::prelude::*;

use crate::schema::cvs;

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


fn create_cv(new_title: &str, new_author: &str, new_body: &str, conn: &mut PgConnection) -> usize {
    use crate::schema::cvs::dsl::*;

    let inserted_cv = NewCV {
        title: new_title,
        author: new_author,
        body: new_body
    };

    diesel::insert_into(cvs)
        .values(&inserted_cv)
        .execute(conn)
        .unwrap_or_else(|e| panic!("Can't insert new value: {:?}", e))
}


fn delete_cv(id: String, conn: &mut PgConnection) -> usize {
    use crate::schema::cvs::dsl::*;

    diesel::delete(cvs.filter(id.eq(id)))
        .execute(conn)
        .unwrap_or_else(|e| panic!("Can't delete current value {:?}", e))
}
