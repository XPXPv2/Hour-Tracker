use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Record {
    pub GRADE: u16,
    pub ALTSTUDENTID: usize,
    pub FIRSTNAME: String,
    pub LASTNAME: String,
    pub MIDDLENAME: String,
}

impl From<Record> for super::InductionCanidate{
    fn from(r: Record) -> Self{
        Self{
            first_name: r.FIRSTNAME.into(),
            middle_inital: r.MIDDLENAME.into(),
            last_name: r.LASTNAME.into(),
            grade: r.GRADE.into(),
            id: r.ALTSTUDENTID.try_into().expect(&format!("Failed to convert Student ID {} to i32", r.ALTSTUDENTID)),
            decision: super::database::NOT_DECIED_STRING.into(),
            catigories: slint::ModelRc::new(slint::VecModel::from(vec![4,0,0,0,0])),
        }
    }
}