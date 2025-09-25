
 // ==============================
// Problem #1: String Concatenation with Borrowing
// ==============================
fn concat_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() {
    // Problem #1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("Problem #1 Result: {}", result); 
    

    // ==============================
    // Problem #2: Clone and Modify
    // ==============================
    fn clone_and_modify(s: &String) -> String {
        let mut cloned = s.clone();
        cloned.push_str("World!");
        cloned
    }

    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Problem #2 Original: {}", s);
    println!("Problem #2 Modified: {}", modified); 

    // ==============================
    // Problem #3: Mutable Reference Sum
    // ==============================
    fn sum(total: &mut i32, low: i32, high: i32) {
        let mut acc = 0;
        for i in low..=high {
            acc += i;
        }
        *total = acc; 
    }

    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Problem #3 Sum from 0 to 100: {}", total); 
    
}
