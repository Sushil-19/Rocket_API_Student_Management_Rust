use rocket::serde::json::Json;
use rocket::State;
use std::sync::Mutex;
use uuid::Uuid;

use crate::models::{CreateStudent, Student, UpdateStudent};

type StudentList = Mutex<Vec<Student>>;

#[get("/students")]
pub fn get_students(data: &State<StudentList>) -> Json<Vec<Student>> {
    let students = data.lock().unwrap();
    Json(students.clone())
}

#[get("/students/<id>")]
pub fn get_student_by_id(data: &State<StudentList>, id: Uuid) -> Option<Json<Student>> {
    let students = data.lock().unwrap();
    students.iter().find(|&student| student.id == id).cloned().map(Json)
}

#[post("/students", data = "<new_student>")]
pub fn create_student(data: &State<StudentList>, new_student: Json<CreateStudent>) -> Json<Student> {
    let mut students = data.lock().unwrap();
    let student = Student {
        id: Uuid::new_v4(),
        name: new_student.name.clone(),
        age: new_student.age,
        department: new_student.department.clone(),
    };
    students.push(student.clone());
    Json(student)
}

#[put("/students/<id>", data = "<updated_student>")]
pub fn update_student(data: &State<StudentList>, id: Uuid, updated_student: Json<UpdateStudent>) -> Option<Json<Student>> {
    let mut students = data.lock().unwrap();
    if let Some(student) = students.iter_mut().find(|student| student.id == id) {
        student.name = updated_student.name.clone();
        student.age = updated_student.age;
        student.department = updated_student.department.clone();
        return Some(Json(student.clone()));
    }
    None
}

#[delete("/students/<id>")]
pub fn delete_student(data: &State<StudentList>, id: Uuid) -> Option<Json<()>> {
    let mut students = data.lock().unwrap();
    if let Some(pos) = students.iter().position(|student| student.id == id) {
        students.remove(pos);
        return Some(Json(()));
    }
    None
}
