use std::collections::HashMap;

struct Student {
    name : String,
    grades : Vec<f64>,
}

impl Student {
    fn new( name : String) -> Student {
        let student = Student{
            name : name,
            grades : Vec::new(),
        };
        student
    }
    fn add_grade(&mut self, grade : f64) {
        self.grades.push(grade);
    }
    fn average(&self){
        let mut average_student = 0.0;
        for grade in self.grades.iter() {
            average_student += grade;
        }
        average_student /= self.grades.len() as f64;
        println!("El promedio del estudiante {} es: {}", self.name, average_student);
    }
}

fn main() {
    let mut students : HashMap<String, Student> = HashMap::new();
    let mut student1 = Student::new("Juan".to_string());
    student1.add_grade(4.0);
    student1.add_grade(5.0);

    let mut student2 = Student::new("Pedro".to_string());
    student2.add_grade(3.5);
    student2.add_grade(4.5);
    student2.add_grade(5.0);

    students.insert(student1.name.clone(), student1);
    students.insert(student2.name.clone(), student2);

    println!("\nPromedios");
    for student in students.values()  {
      student.average();
    }
}
