use core::panic;
use actix_web::web::{Json, Path};
use actix_web::{get, HttpResponse, web, post, delete};
use diesel::{prelude::*, Queryable};
use diesel::result::Error;
use serde::de::{Visitor, SeqAccess};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use crate::schema::cvs;
use crate::models::connections::establish_connection;
use serde::de;



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


impl<'de> Deserialize<'de> for CV {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
            {
                enum Field { Ids, Titles, Bodies, Authors }
                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>, {
                                struct FieldVisitor;
                                impl<'de> Visitor<'de> for FieldVisitor {
                                    type Value = Field;
                                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                                        formatter.write_str("'id' or 'title' or 'body' or 'author'")
                                    }
                                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                                        where
                                            E: serde::de::Error, {
                                        match v {
                                            "id" => Ok(Field::Ids),
                                            "title" => Ok(Field::Titles),
                                            "body" => Ok(Field::Bodies),
                                            "author" => Ok(Field::Authors),
                                            _ => Err(serde::de::Error::unknown_field(v, FIELDS)),
                                        }
                                    }
                                }
                                deserializer.deserialize_identifier(FieldVisitor)
                    }
                }
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = CV;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("struct CV")
                    }
                    fn visit_seq<V>(self, mut seq: V) -> Result<CV, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let ids = seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let titles = seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        let bodies = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                        let authors = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                        Ok(CV::new(ids, titles, bodies, authors))
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                        where
                            A: de::MapAccess<'de>, {
                        let mut id = None;
                        let mut title = None;
                        let mut body = None;
                        let mut author = None;
                        while let Some(key) = map.next_key()?  {
                            match key {
                                Field::Ids => {
                                    if id.is_some() {
                                        return Err(de::Error::duplicate_field("id"))
                                    }
                                    id = Some(map.next_value()?);
                                },
                                Field::Titles => {
                                    if title.is_some() {
                                        return Err(de::Error::duplicate_field("title"))
                                    }
                                    title = Some(map.next_value()?);
                                },
                                Field::Bodies => {
                                    if body.is_some() {
                                        return Err(de::Error::duplicate_field("body"))
                                    }
                                    body = Some(map.next_value()?);
                                },
                                Field::Authors => {
                                    if author.is_some() { 
                                        return Err(de::Error::duplicate_field("author"))
                                    }
                                    author = Some(map.next_value()?);
                                }
                            }
                        }
                        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                        let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
                        let body = body.ok_or_else(|| de::Error::missing_field("body"))?;
                        let author = author.ok_or_else(|| de::Error::missing_field("author"))?;
                        Ok(CV::new(id, title, body, author))
                    }
        
                }
                const FIELDS: &'static [&'static str] = &["id", "title", "body", "author"];
                deserializer.deserialize_struct("CV", FIELDS, FieldVisitor)
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
        .unwrap_or_else(|e| panic!("can't create new cv: {:?}", e))
}


fn delete_cv(current_id: i32) -> usize {
    use crate::schema::cvs::dsl::*;

    // TODO Change to DATA<POOL>
    let connection = &mut establish_connection();

    diesel::delete(cvs.filter(id.eq(current_id)))
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


#[post("/new_cvs")]
pub async fn create(CV_req: Json<Option<CV>>) -> HttpResponse {
    let n: CV = CV_req.into_inner().unwrap();
    let cv = web::block(move || 
        create_cv(n.title.as_str(), n.author.as_str(), n.body.as_str())).await;

    match cv {
        Ok(c) => 
        {
            HttpResponse::Created()
            .content_type("Application/json")
            .json(c)
        },
        _ => {
            HttpResponse::NoContent().await.unwrap()
        },
    }
}

#[delete("/cv/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    let _ = web::block(move || delete_cv(path.0.as_str().parse().unwrap())).await;

    HttpResponse::NoContent()
        .content_type("Application/json")
        .await
        .unwrap()

}
