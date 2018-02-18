#[derive(Debug)]
pub struct Aqi {
    pub value: i16,
    pub time: ::chrono::NaiveDateTime,
}
