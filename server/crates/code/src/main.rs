mod render;

use clap::Parser;

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
    let args = Args::parse();
    let url = args.url;
    let tables = args.tables;
    let module = args.module.unwrap_or_else(|| "MyModule".to_string());
    let output = args.path.unwrap_or_else(|| "./".to_string());
    for table_name in tables {
        println!("Generating code for table {} in database {} with module name {} and output path {}", table_name, url, module, output);
        //TODO: 获取数据库表信息(model使用diesel生成)

        //TODO: 构造模板所需数据
        let data = render::make_data();
        //渲染模板
        let result = render::render(data);
        match result {
            Ok(_) => (),
            Err(e) => println!("{}", e)
        }
    }
    println!("done")
}