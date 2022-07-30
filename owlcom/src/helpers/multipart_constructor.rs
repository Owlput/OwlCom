use std::path::Path;

pub async fn file(path: &Path) -> Result<reqwest::multipart::Part, std::io::Error> {
    let file = match tokio::fs::File::open(path).await {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };
    let part = reqwest::multipart::Part::stream(file);
    Ok(part)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::file;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test() {
        file(Path::new("./Cargo.toml")).await.unwrap();
    }
}
