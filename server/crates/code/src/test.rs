#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let s1 = "hello world";
        let s2 = "hello world";
        assert_eq!(s2, s1);
    }

    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        use std::collections::HashMap;
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }
}