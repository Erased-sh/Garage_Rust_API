use actix_web::web::Json;
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
        state.serialize_field("author", &self.author)?;
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
    HttpResponse::Ok()
        .content_type("Aplication/json")
        .json(res)
}

// TODO Refactor usize output of function 
#[post("/cvs")]
pub async fn create(CV_req: Json<Option<CV>>) -> HttpResponse {
    let connection = &mut establish_connection();
    let n: CV = CV_req.into_inner().unwrap();
    let cv = web::block(move || 
        create_cv(n.title.as_str(), n.author.as_str(), n.body.as_str())).await;

    match tweet {
        Ok(tweet) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}