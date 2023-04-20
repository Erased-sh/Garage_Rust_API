use RUST_REST_APIv2::cv::CV;
use RUST_REST_APIv2::schema;
use RUST_REST_APIv2::models::connections::establish_connection;

extern crate diesel;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};
use dotenvy::dotenv;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // set up database connection pool
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|e| panic!("Error with get env by name 'DATABASE_URL': {:?}", e));

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(pool.clone())

            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())

            
            // register HTTP requests handlers
            //
            // EXAMPLE
            // .service(NAME_OF_SERVICE)
            //
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}




