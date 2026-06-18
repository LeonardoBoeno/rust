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

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Student => write!(f, "Student"),
            Role::Teacher => write!(f, "Teacher"),
        }
    }
}

enum Profile {
    BeginnerStudent,
    BeginnerTeacher,
    AdvancedStudent,
    AdvancedTeacher,
}

impl Profile {
    fn new(stage: Stage, role: Role) -> Self {
        match (stage, role) {
            (Stage::Beginner, Role::Student) => Profile::BeginnerStudent,
            (Stage::Beginner, Role::Teacher) => Profile::BeginnerTeacher,
            (Stage::Advanced, Role::Student) => Profile::AdvancedStudent,
            (Stage::Advanced, Role::Teacher) => Profile::AdvancedTeacher,
        }
    }

    fn describe(&self) -> &'static str {
        match self {
            Profile::BeginnerStudent => "A beginner student starting their journey",
            Profile::BeginnerTeacher => "A beginner teacher learning to teach",
            Profile::AdvancedStudent => "An advanced student mastering the subject",
            Profile::AdvancedTeacher => "An advanced teacher guiding others",
        }
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

fn main() {
    use Stage::*;
    use Role::*;

    let stage = Beginner;
    let role = Student;

    let profile = Profile::new(stage, role);

    println!("Profile: {}", profile);
}
