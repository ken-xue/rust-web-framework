use serde_json::value::{Map, Value as Json};
use std::error::Error;
use std::fs;
use std::fs::{File};
use handlebars::{to_json, Handlebars};
use crate::{repo, template};

pub fn render(template :String,module :String,table : repo::Table,output :String) -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    // 注册模板
    handlebars.register_template_file("template", template::TEMPLATE_DIR.to_owned()+"/"+&template).unwrap();
    // 输出文件
    let mut output_path = String::from(output);
    if !output_path.ends_with('/') {
        output_path.push('/');
    }
    // 模块目录
    output_path.push_str(module.as_str());
    output_path.push('/');
    // 本身模块
    output_path.push_str(table.remove_prefix_table_name.as_str());
    output_path.push('/');
    // 文件名
    let mut parts_iter = template.splitn(2, '.');
    let name = parts_iter.next().unwrap_or(&template);
    output_path.push_str(name);
    output_path.push_str(".rs");
    // 创建目录
    if let Some(parent_dir) = std::path::Path::new(output_path.as_str()).parent() {
        fs::create_dir_all(parent_dir)?;
    }
    // 创建文件
    let mut output_file = File::create(output_path)?;
    // 构造数据
    let data = build_render_data(table);
    // 渲染模板
    handlebars.render_to_write("template", &data, &mut output_file)?;
    Ok(())
}

// produce some data
pub fn build_render_data(table : repo::Table) -> Map<String, Json> {
    let mut data = Map::new();
    data.insert("table".to_string(), to_json(table));
    data
}
