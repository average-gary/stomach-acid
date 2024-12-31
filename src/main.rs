
use hyper::{
    body::HttpBody, client::HttpConnector, header, Client, Method, Request, Response,
};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let username = "root";
    let password = "root";
    let host = "10.30.37.25";
    let uri = hyper::Uri::builder()
        .authority(host)
        .path_and_query("/cgi-bin/summary.cgi")
        .scheme("http")
        .build()
        .unwrap();
    let builder = Client::builder();
    let http = HttpConnector::new();
    let client = builder.build::<HttpConnector, String>(http);
    let response = client.get(uri.clone()).await.unwrap();
    if response.status().as_u16() == 401 && response.headers().get("www-authenticate").is_some() {
        let parts = response
            .headers()
            .get("www-authenticate")
            .unwrap()
            .to_str()
            .unwrap();
        let parts: Vec<&str> = parts.split(",").collect();
        let mut realm = "";
        let mut nonce = "";
        for part in parts {
            if part.contains("realm") {
                realm = part.split("=").collect::<Vec<&str>>()[1].trim_matches('"');
            }
            if part.contains("nonce") {
                nonce = part.split("=").collect::<Vec<&str>>()[1].trim_matches('"');
            }
            println!("{}", part);
        }
        let ha1 = format!("{:x}", md5::compute(format!("{}:{}:{}", username, realm, password)));
        let ha2 = format!("{:x}", md5::compute(format!("GET:{}", uri.path())));
        let cnonce = "00000001";
        let response = format!("{:x}", md5::compute(format!("{}:{}:{}:{}:{}:{}", ha1, nonce, "00000001", cnonce, "auth", ha2)));

        let auth = format!(
            "Digest username=\"{}\", realm=\"{}\", nonce=\"{}\", uri=\"{}\", response=\"{}\", cnonce=\"{}\", nc=\"00000001\", qop=\"auth\"",
            username,
            realm,
            nonce,
            uri.path(),
            response,
            cnonce
        );

        let request = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header(header::AUTHORIZATION, auth)
            .body("".to_string())
            .expect("request builder");
        println!("{:?}", request);

        let mut response = client.request(request).await.unwrap();
        let body = response.into_body();
        println!("{:?}", body.collect().await.unwrap());
    }
    Ok(())
}
