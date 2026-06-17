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

impl Stage {
    fn is_beginner(&self) -> bool {
        matches!(self, Stage::Beginner)
    }

    fn is_advanced(&self) -> bool {
        matches!(self, Stage::Advanced)
    }

    fn level(&self) -> u8 {
        match self {
            Stage::Beginner => 1,
            Stage::Advanced => 2,
        }
    }
}

impl Role {
    fn is_student(&self) -> bool {
        matches!(self, Role::Student)
    }

    fn is_teacher(&self) -> bool {
        matches!(self, Role::Teacher)
    }

    fn description(&self) -> &'static str {
        match self {
            Role::Student => "Learns new things",
            Role::Teacher => "Teaches others",
        }
    }
}

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
    use Stage::{Beginner, Advanced};
    use Role::*;

    let stage = Beginner;
    let role = Student;

    println!("Stage: {}", stage);
    println!("Role: {}", role);

    // Testando métodos
    println!("Is beginner? {}", stage.is_beginner());
    println!("Is advanced? {}", stage.is_advanced());
    println!("Stage level: {}", stage.level());

    println!("Is student? {}", role.is_student());
    println!("Is teacher? {}", role.is_teacher());
    println!("Role description: {}", role.description());

    match stage {
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}
