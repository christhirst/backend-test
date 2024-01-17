use leptos::*;
use crate::model::data::{Data, Datas};


#[server(Converse, "/api")]
 pub async fn converse( prompt: Datas) -> Result<String, ServerFnError> {
todo!()
} 
