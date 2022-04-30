use anyhow::Result;

fn main() -> Result<()> {
    if let Some(path) = std::env::args().nth(1) {
        let spec = apple_bloom::from_path(path)?;
        /*for (path, op) in spec.paths {
            println!("{}", path);
            println!("{:#?}", op);
        }
        for (name, definition) in spec.definitions {
            println!("{}", name);
            println!("{:#?}", definition);
        }*/
        println!("{}", apple_bloom::to_json(&spec)?);
    }
    Ok(())
}
