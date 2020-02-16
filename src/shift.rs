use std::string;

//mod types;

pub struct Shift<'a> {
    name: &'a str,
    week: i32,
}

impl string::ToString for Shift {
    fn to_string(&self) -> String {
        format!("{},{}", self.week, self.name)
    }
}
