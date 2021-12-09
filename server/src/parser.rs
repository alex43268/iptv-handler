use std::env;

pub async fn parser() {
    let m3u = match env::var("M3U") {
        Ok(val) => val,
        Err(e) => e.to_string(),
    };

    let res = match get_m3u(&m3u).await {
        Ok(res) => res,
        Err(e) => e.to_string(),
    };

    println!("{:?}", res);
    println!("{:?}", res);
}

async fn get_m3u(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = surf::get(url).await?.body_string().await?;
    println!("{:?}", res);
    Ok("".to_string())
}