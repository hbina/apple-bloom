#[cfg(test)]
pub mod test_util {
    use serde::de::DeserializeOwned;
    use std::path::PathBuf;

    pub fn parse_from_file<T: DeserializeOwned>(
        string: impl Into<PathBuf>,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let reader =
            std::io::BufReader::new(std::fs::OpenOptions::new().read(true).open(string.into())?);
        let result: T = serde_yaml::from_reader(reader)?;
        Ok(result)
    }

    pub fn get_all_file_paths_in_directory(
        path: impl Into<String>,
    ) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let path = path.into();
        Ok(std::fs::read_dir(path)
            .unwrap()
            .map(|res| res.unwrap().path())
            .filter(|path| path.is_file())
            .collect::<Vec<_>>())
    }
}
