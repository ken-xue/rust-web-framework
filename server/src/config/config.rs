use tokio::fs::File;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};

fn load_file(file: &str) -> Vec<Yaml>{
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    docs
}