use clap::Parser;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    path::PathBuf,
};

#[derive(Parser)]
struct Opts {
    #[clap(short, long, required = true)]
    from: String,

    #[clap(short, long, required = true)]
    to: String,

    #[clap(short, long, required = true)]
    wait: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    println!("Moving from {} to {}", opts.from, opts.to);

    let _ = fs::create_dir(&opts.to);

    let dest_dir = PathBuf::from(opts.to);
    let mut map: HashMap<PathBuf, u32> = HashMap::new();

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let mut files: HashSet<PathBuf> = HashSet::new();

        for entry in fs::read_dir(opts.from.clone())? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let count = map.entry(path.clone()).or_insert(0);
            *count += 1;

            if *count < opts.wait {
                files.insert(path.clone());
                continue;
            }

            let file_name = path.file_name().unwrap().to_string_lossy();
            let new_path = dest_dir.join(file_name.into_owned());
            fs::copy(&path, &new_path)?;
            fs::remove_file(&path)?;
            println!("move: {:?}", new_path);
        }

        map.retain(|k, _| files.contains(k));
    }
}
