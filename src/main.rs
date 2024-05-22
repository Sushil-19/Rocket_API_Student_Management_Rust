#[macro_use] extern crate rocket;

mod handlers;
mod models;

use std::sync::Mutex;
use handlers::*;
use models::Student;

#[launch]
fn rocket() -> _ {
    let student_list: Mutex<Vec<Student>> = Mutex::new(Vec::new());

    rocket::build()
        .manage(student_list)
        .mount("/", routes![
            get_students,
            get_student_by_id,
            create_student,
            update_student,
            delete_student
        ])
}
