#[macro_use] extern crate rocket;

mod handlers;
mod models;

use handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_students,
            get_student_by_id,
            create_student,
            update_student,
            delete_student
        ])
}
