use rocket::serde::json::Json;
use rocket::State;
use std::sync::Mutex;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use uuid::Uuid;

use crate::models::{CreateStudent, Student, UpdateStudent};

const FILE_PATH: &str = "students.json";

fn read_students_from_file() -> io::Result<Vec<Student>> {
    let file = File::open(FILE_PATH).or_else(|_| File::create(FILE_PATH))?;
    let reader = BufReader::new(file);
    let students: Vec<Student> = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);
    Ok(students)
}

fn write_students_to_file(students: &[Student]) -> io::Result<()> {
    let file = File::create(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, students)?;
    Ok(())
}

#[get("/students")]
pub fn get_students() -> io::Result<Json<Vec<Student>>> {
    let students = read_students_from_file()?;
    Ok(Json(students))
}

#[get("/students/<id>")]
pub fn get_student_by_id(id: Uuid) -> io::Result<Option<Json<Student>>> {
    let students = read_students_from_file()?;
    Ok(students.into_iter().find(|student| student.id == id).map(Json))
}

#[post("/students", data = "<new_student>")]
pub fn create_student(new_student: Json<CreateStudent>) -> io::Result<Json<Student>> {
    let mut students = read_students_from_file()?;
    let student = Student {
        id: Uuid::new_v4(),
        name: new_student.name.clone(),
        age: new_student.age,
        department: new_student.department.clone(),
    };
    students.push(student.clone());
    write_students_to_file(&students)?;
    Ok(Json(student))
}

#[put("/students/<id>", data = "<updated_student>")]
pub fn update_student(id: Uuid, updated_student: Json<UpdateStudent>) -> io::Result<Option<Json<Student>>> {
    let mut students = read_students_from_file()?;
    if let Some(student) = students.iter_mut().find(|student| student.id == id) {
        student.name = updated_student.name.clone();
        student.age = updated_student.age;
        student.department = updated_student.department.clone();
        
        // Clone the student to be returned
        let updated_student_clone = student.clone();
        
        // Drop the mutable borrow before the immutable borrow
        drop(student);
        
        write_students_to_file(&students)?;
        return Ok(Some(Json(updated_student_clone)));
    }
    Ok(None)
}

#[delete("/students/<id>")]
pub fn delete_student(id: Uuid) -> io::Result<Option<Json<()>>> {
    let mut students = read_students_from_file()?;
    if let Some(pos) = students.iter().position(|student| student.id == id) {
        students.remove(pos);
        write_students_to_file(&students)?;
        return Ok(Some(Json(())));
    }
    Ok(None)
}
