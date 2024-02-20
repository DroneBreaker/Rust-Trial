pub trait Overview {
    fn overview(&self) -> String {
        String::from("This is a Rust course")
    }
}

pub struct Course {
    pub(crate) headline: String,
    pub author: String,
}

pub struct AnotherCourse {
    pub headline: String,
    pub author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

// const course1 = Course {headline: String::from("headline"), author: String::from("Browne")};
// let course2 = Course {headline: String::from("Another headline"), author: String::from(" Another Browne")};
