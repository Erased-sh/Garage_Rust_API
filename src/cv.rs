use actix_web::{get, HttpResponse, web};
use diesel::{prelude::*, Queryable};
use diesel::result::Error;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use crate::models::Response;
use crate::schema::cvs;
use crate::models::connections::establish_connection;



#[derive(Queryable)]
pub struct CV {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author: String
}

impl Serialize for CV {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("CV", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("body", &self.body)?;
        state.serialize_field("author", &self.author);
        state.end()
    }
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

    // TODO Change to DATA<POOL>
    let connection = &mut establish_connection();

    diesel::insert_into(cvs)
        .values(&inserted_cv)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Can't insert new value: {:?}", e))
}


fn delete_cv(id: String) -> usize {
    use crate::schema::cvs::dsl::*;

    // TODO Change to DATA<POOL>
    let connection = &mut establish_connection();

    diesel::delete(cvs.filter(id.eq(id)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Can't delete current value {:?}", e))
}


fn list_cv() -> Result<Vec<CV>, Error> {
    use crate::schema::cvs::dsl::*;

    let connection = &mut establish_connection();

    cvs.load::<CV>(connection)
    // {
    //     Ok(res) => res,
    //     Err(_) => vec![]
    // };

    // Ok(Response { results: _cvs })
}

#[get("/cvs")]
pub async fn list() -> HttpResponse {
    let res = web::block(move || list_cv())
        .await
        .unwrap()
        .unwrap();
        // .map(|cv| HttpResponse::Ok().json(cv));
        // .map_err(|_| HttpResponse::InternalServerError());
    HttpResponse::Ok()
        .content_type("Aplication/json")
        .json(res)


    // let mut arr_cvs = web::block(move || list_cv(50))
    //     .await
    //     .map(|cv| HttpResponse::Ok().json(cv))
    //     .unwrap();

    // HttpResponse::Ok()
    //     .content_type("application/json")
    //     .json(arr_cvs)
}