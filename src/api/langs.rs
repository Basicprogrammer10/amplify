use std::{collections::HashMap, os::windows::raw};
use std::fs;

use afire::{Content, Method, Response, Server};
use lazy_static::lazy_static;
use serde_json::{from_str, Value};

use crate::{App, Arc};

lazy_static! {
    pub static ref LANGS: HashMap<String, (String, String)> = {
        let raw_langs: Value = from_str(&fs::read_to_string("langs/languages.json").unwrap())
            .expect("Error parsing langs/languages.json");
        let mut langs = HashMap::new();

        for i in raw_langs.as_array().unwrap() {
            langs.insert(
                i.get("name").unwrap().as_str().unwrap().to_owned(),
                (
                    i.get("imageName").unwrap().as_str().unwrap().to_owned(),
                    i.get("sourcePath").unwrap().as_str().unwrap().to_owned(),
                ),
            );
        }

        langs
    };
}

pub fn attatch(server: &mut Server, _: Arc<App>) {
    let langs_json = format!(
        "[\"{}\"]",
        LANGS
            .iter()
            .map(|x| x.0.to_owned())
            .collect::<Vec<_>>()
            .join("\",\"")
    );

    server.route(Method::GET, "/api/langs", move |_| {
        Response::new().text(&langs_json).content(Content::JSON)
    });
}
