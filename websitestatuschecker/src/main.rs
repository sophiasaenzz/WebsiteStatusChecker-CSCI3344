//use ureq;
//create a concurrent website monitoring system that can check the status of multiple websites similtaneously
use ureq;
use std::time::Duration;
use std::thread;
use std::sync::mpsc; //for channels communciation between threads
use std::fs;
use std::io::Write;
//use serde::{Deserialize, Serialize};


//create a struct to hold the website status
#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    error: Option<String> //error message
}


impl WebsiteStatus { //constructor
    fn new(url: String, status: Result<u16, String>, response_time: Duration, error: Option<String>) -> WebsiteStatus {
        WebsiteStatus {
            url,
            status,
            response_time,
            error,
        }
    }
}


fn check_website(url: &str, _timeout: Duration) -> WebsiteStatus {
    let start = std::time::Instant::now();
    let mut error = None;

    //use ureq agent to hold connection 
    let agent = ureq::AgentBuilder::new().timeout_read(Duration::from_secs(5)).timeout_connect(Duration::from_secs(5)).build();

    let status = match agent.get(url).call() { //check if the response is ok
        Ok(response) => Ok(response.status()),
        Err(e) => {
            error = Some(e.to_string());
            Err(e.to_string())
        }

    };

    //let response = ureq::get(url).call();
    let response_time = start.elapsed();
    WebsiteStatus::new(url.to_string(), status, response_time, error)
}


fn main() {
    let (tx, rx) = mpsc::channel();
    let timeout = Duration::from_secs(5);

    /*let urls = vec![
        "https://github.com/sophiasaenzz/WebsiteStatusChecker-CSCI3344",
        "http://www.youtube.com",
        "http://www.facebook.com",
        "http://www.baidu.com",
        "http://www.yahoo.com",
        "http://www.amazon.com",
    ]; */

    //read the urls from a file
    let file = fs::read_to_string("urls.txt").unwrap();
    let urls: Vec<&str> = file.lines().collect();


    let mut handles = vec![];

    for url in urls {
        let tx = tx.clone();
        let url = url.to_string();

        let handle = thread::spawn(move || {
            let website_status = check_website(&url, timeout);
            tx.send(website_status).unwrap();
            //println!("{:?}", website_status);
        });
        handles.push(handle);
    }

    //wait for all threads to finish, close sender
    //communication between threads
    drop(tx);
    let mut results = vec![];

    for received in rx {
        results.push(received);
    }


    for handle in handles {
        handle.join().unwrap();
    }

    //write the status to a file
    let mut file = fs::File::create("status.txt").unwrap();
    for status in results {
        println!("{:?}", status);

        writeln!(file, "{:?}", status).unwrap();
    }


    println!("All threads have finished");

}
