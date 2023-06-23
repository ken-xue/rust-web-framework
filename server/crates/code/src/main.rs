use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Database connection string
    #[arg(short, long)]
    url: String,

    /// Name of the table to generate
    #[arg(short, long)]
    table: String,

    /// Name of the module for the generated code
    #[arg(short, long)]
    module: Option<String>,

    /// Path to the output file for the generated code
    #[arg(short, long)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let db_conn = args.url;
    let table_name = args.table;
    let module_name = args.module.unwrap_or_else(|| "MyModule".to_string());
    let output_path = args.path.unwrap_or_else(|| "output.rs".to_string());

    // TODO: Use the arguments to generate the code for the specified table
    println!("Generating code for table {} in database {} with module name {} and output path {}", table_name, db_conn, module_name, output_path);
}