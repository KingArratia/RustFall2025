// main.rs

struct Student {
    name: String,
    major: String,
}

impl Student {
    fn new(name: &str, major: &str) -> Self {
        Student {
            name: name.to_string(),
            major: major.to_string(),
        }
    }

    
    fn set_major(&mut self, new_major: &str) {
        self.major = new_major.to_string();
    }

    
    fn get_major(&self) -> &str {
        &self.major
    }
}

fn main() {
    
    let mut st = Student::new("Brandon", "Computer Science");

    println!("Name: {}", st.name);
    println!("Major: {}", st.get_major());

    
    st.set_major("Criminal Justice");

    println!("Updated Major: {}", st.get_major());
}
