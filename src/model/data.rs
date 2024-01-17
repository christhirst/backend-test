use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Datas {
    pub data: Vec<Data>,
}

impl Datas {
    pub fn new() -> Datas {
        Datas { data: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Data {
    pub user: bool,
    pub text: String,
}
