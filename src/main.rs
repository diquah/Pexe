use std:: {
    env,
    fs::metadata
};

mod build;
mod run;
mod format_spec;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_path = &args[1];
    let target_metadata = metadata(target_path).unwrap();

    if target_metadata.is_dir() {
        build::build(target_path);
    } else if target_metadata.is_file() {
        run::run(target_path);
    } else if target_metadata.is_symlink() {
        unimplemented!();
    }
    
}