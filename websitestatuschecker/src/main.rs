//use ureq;
//create a concurrent website monitoring system that can check the status of multiple websites similtaneously

use ureq;
use std::time::Duration;
use std::thread;
use chrono::{DateTime, Utc};
use std::sync::mpsc; //for channels communciation between threads

//must accept a list of website urls for monitoring
//must support concurrent checking of websites
//must implement configurable timeout for each request
//most collect and report http status code, response time, and error encountered

//implemention must use rust threading system
//must use channels for communication between threads
//must implement proper error handling
//must support graceful shutdown

//have at least 2 threads running concurrently

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}


impl WebsiteStatus {
    fn new(url: String, status: Result<u16, String>, response_time: Duration, timestamp: DateTime<Utc>) -> WebsiteStatus {
        WebsiteStatus {
            url,
            status,
            response_time,
            timestamp,
        }
    }
}


fn check_website(url: &str) -> WebsiteStatus {
    let start = std::time::Instant::now();
    let timestamp = Utc::now();
    //use ureq agent to hold connection 
    let agent = ureq::AgentBuilder::new().timeout_read(Duration::from_secs(5)).timeout_connect(Duration::from_secs(5)).build();

    let status = match agent.get(url).call() { //check if the response is ok
        Ok(response) => Ok(response.status()),
        Err(e) => Err(e.to_string()),
    };

    //let response = ureq::get(url).call();
    let response_time = start.elapsed();
    WebsiteStatus::new(url.to_string(), status, response_time, timestamp)
}


fn main() {
    let urls = vec![
        "https://www.google.com",
        "https://www.facebook.com",
        "https://www.twitter.com",
        "https://www.reddit.com",
        "https://www.youtube.com",
    ];

    let mut handles = vec![];

    for url in urls {
        let handle = thread::spawn(move || {
            let website_status = check_website(url);
            println!("{:?}", website_status);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have finished");
}
