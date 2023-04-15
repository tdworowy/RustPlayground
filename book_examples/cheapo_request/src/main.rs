use surf::{Client, Error};

pub async fn many_requests(urls: &[String]) -> Vec<Result<String, Error>> {
    let client = Client::new();
    let mut handles = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

fn main() {
    let requests = &[
        "http://example.com".to_string(),
        "https://www.readbean.com".to_string(),
        "https://en.wikipedia.org/wiki/Main_page".to_string(),
    ];
    let results = async_std::task::block_on(many_requests(requests));

    for result in results {
        match result {
            Ok(response) => println!("*** {}\n", response),
            Err(err) => println!("error: {}\n", err),
        }
    }
}
