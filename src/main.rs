use RUST_REST_APIv2::cv::CV;
use RUST_REST_APIv2::schema;
use RUST_REST_APIv2::models::connections::establish_connection;

use diesel::RunQueryDsl;


fn main() {
    use schema::cvs::dsl::*;

    let connection = &mut establish_connection();
    let results = cvs
        .load::<CV>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}






