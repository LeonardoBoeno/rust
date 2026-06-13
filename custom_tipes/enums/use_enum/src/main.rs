// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::fmt;

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

// Implement Display for Stage
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stage::Beginner => write!(f, "Beginner"),
            Stage::Advanced => write!(f, "Advanced"),
        }
    }
}

// Implement Display for Role
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Student => write!(f, "Student"),
            Role::Teacher => write!(f, "Teacher"),
        }
    }
}

fn main() {
    // Explicitly `use` each name so they are available without manual scoping.
    use Stage::{Beginner, Advanced};
    // Automatically `use` each name inside `Role`.
    use Role::*;

    let stage = Beginner;
    let role = Student;

    // Agora você pode imprimir diretamente com {}
    println!("Stage: {}", stage);
    println!("Role: {}", role);

    match stage {
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}
