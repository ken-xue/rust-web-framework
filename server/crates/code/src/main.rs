mod render;
mod repo;

use std::env;
use clap::Parser;
use diesel::MysqlConnection;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Database connection string
    #[arg(short, long)]
    url: String,

    /// Name of the tables to generate
    #[arg(short, long)]
    tables: Vec<String>,

    /// Name of the module for the generated code
    #[arg(short, long)]
    module: Option<String>,

    /// Path to the output file for the generated code
    #[arg(short, long)]
    path: Option<String>,

    /// Use template file for the generated code , default all template
    #[arg(long)]
    template: Option<String>,

    /// Need remove prefix table name
    #[arg(long)]
    prefix: Option<String>,
}

fn main() {
    env::set_var("RUST_LOG", "trace");
    let args = Args::parse();
    let url = args.url;
    let tables = args.tables;
    let module = args.module.unwrap_or_else(|| "MyModule".to_string());
    let output = args.path.unwrap_or_else(|| "./".to_string());
    //模板
    let templates = vec![
        "model.hbs",
    ];
    // 获取数据库连接
    let conn  = &mut repo::establish_connection(url.clone());
    // 遍历所有数据库表
    for table_name in tables {
        println!("Generating code for table {} in database {} with module name {} and output path {}", table_name, url, module, output);
        for template in templates.clone().into_iter() {
            let table = repo::get_table_info(conn,table_name.as_str());
            match render::render(template.to_string(),table,output.to_string()) {
                Ok(_) => (),
                Err(e) => println!("{}", e)
            }
        }
    }
    println!("done")
}