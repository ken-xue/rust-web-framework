use serde::Serialize;
use serde_json::value::{self, Map, Value as Json};
use std::error::Error;
use std::fs::{File, metadata};
use std::io::{Read, Write};

use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};
use crate::repo;

static TYPES: &'static str = "serde_json";

// define some data
#[derive(Serialize)]
pub struct Field {
    name: String,
    data_type: String,
}

pub fn render(template :String,table : repo::Table,output :String) -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("template", "/Users/ithpnb04101/CLionProjects/rust-web-framework/server/crates/code/src/template/model.hbs").unwrap();
    let mut output_file = File::create(output+"model.rs")?;
    // 构造数据
    let data = build_render_data(table);
    // 渲染模板
    handlebars.render_to_write("template", &data, &mut output_file)?;
    Ok(())
}

// produce some data
pub fn build_render_data(table : repo::Table) -> Map<String, Json> {
    let mut data = Map::new();

    data.insert("year".to_string(), to_json("2015"));

    let teams = vec![
        Field {
            name: "Jiangsu Suning".to_string(),
            data_type: "43u16".to_string(),
        },
    ];

    data.insert("teams".to_string(), to_json(&teams));
    data.insert("engine".to_string(), to_json(TYPES));
    data
}
