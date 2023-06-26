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
}

fn main() {
    let args = Args::parse();
    let url = args.url;
    let tables = args.tables;
    let module = args.module.unwrap_or_else(|| "MyModule".to_string());
    let output = args.path.unwrap_or_else(|| "output.rs".to_string());
    for table_name in tables {
        println!("Generating code for table {} in database {} with module name {} and output path {}", table_name, url, module, output);
        // TODO: Generate the code for the current table
        //./
        // ├── domain.rs.hbs
        // ├── handler.rs.hbs
        // ├── mod.rs.hbs
        // ├── model.rs.hbs
        // ├── repo.rs.hbs
        // ├── request.rs.hbs
        // └── response.rs.hbs
    }
    render::render();
    println!("done")
}