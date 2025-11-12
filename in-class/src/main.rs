use std::fmt::Display;

// ----- Shared Trait -----
trait ShowInfo {
    fn show_info(&self);
}

// ----- Shared Struct -----
struct StudentData {
    name: String,
    major: String,
    gpa: f32,
}

// ----- Undergrad -----
struct Undergrad {
    data: StudentData,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!(
            "Undergrad: {} | Major: {} | GPA: {:.2}",
            self.data.name, self.data.major, self.data.gpa
        );
    }
}

// ----- Grad -----
struct Grad {
    data: StudentData,
    thesis: String,
}

impl ShowInfo for Grad {
    fn show_info(&self) {
        println!(
            "Grad: {} | Major: {} | GPA: {:.2} | Thesis: {}",
            self.data.name, self.data.major, self.data.gpa, self.thesis
        );
    }
}

// ----- Enrollment -----
struct Enrollment {
    students: Vec<Box<dyn ShowInfo>>,
}

impl Enrollment {
    fn new() -> Self {
        Enrollment { students: Vec::new() }
    }

    fn add_student<T: ShowInfo + 'static>(&mut self, student: T) {
        self.students.push(Box::new(student));
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

// ----- Demonstration -----
fn main() {
    let undergrad = Undergrad {
        data: StudentData {
            name: "Brandon".to_string(),
            major: "Computer Engineering".to_string(),
            gpa: 3.8,
        },
    };

    let grad = Grad {
        data: StudentData {
            name: "Mia".to_string(),
            major: "Physics".to_string(),
            gpa: 3.9,
        },
        thesis: "Quantum Computing Applications".to_string(),
    };

    let mut enrollment = Enrollment::new();
    enrollment.add_student(undergrad);
    enrollment.add_student(grad);

    enrollment.show_all();
}
