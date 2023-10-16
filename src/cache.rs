pub async fn cache<T, Fut, E>(
    cache_file: &str,
    func: impl FnOnce() -> Fut,
) -> std::result::Result<T, E>
where
    Fut: std::future::Future<Output = std::result::Result<T, E>>,
    T: serde::de::DeserializeOwned,
    T: serde::Serialize,
{
    let data = if std::path::Path::new(&cache_file).exists() {
        log::debug!("Reading cache {}", cache_file);
        let cache_contents = std::fs::read(cache_file)
            .unwrap_or_else(|err| panic!("Error reading cache file: {}. {:?}", cache_file, err));
        bincode::deserialize(&cache_contents[..]).expect("While deserializing cache")
    } else {
        log::debug!("No cache found {}", cache_file);
        let eval_result = func().await?;
        let cache_contents =
            bincode::serialize(&eval_result).expect("While serializing func eval result");
        std::fs::write(cache_file, cache_contents).expect("While saving cache");
        eval_result
    };
    Ok(data)
}

#[cfg(test)]
mod tests {
    use crate::{async_test, cache::cache};

    async_test!(test_cache, {
        let res = cache("titanic", || async {
            println!("Running function");
            let resp = reqwest::get(
                "https://web.stanford.edu/class/archive/cs/cs109/cs109.1166/stuff/titanic.csv",
            )
            .await?;
            resp.text().await
        })
        .await;
        println!("{}", res.unwrap());
    });
}
