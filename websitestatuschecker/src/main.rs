//use ureq;
//create a concurrent website monitoring system that can check the status of multiple websites similtaneously
use ureq;
use std::time::Duration;
use std::thread;
use std::sync::mpsc; //for channels communciation between threads


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


fn check_website(url: &str, timeout: Duration) -> WebsiteStatus {
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

    let urls = vec![
        "https://github.com/sophiasaenzz/WebsiteStatusChecker-CSCI3344",
        "http://www.youtube.com",
        "http://www.facebook.com",
        "http://www.baidu.com",
        "http://www.yahoo.com",
        "http://www.amazon.com",
        "http://www.wikipedia.org",
        "http://www.qq.com",
        "http://www.google.co.in",
        "http://www.twitter.com",
        "http://www.live.com",
        "http://www.taobao.com",
        "http://www.bing.com",
        "http://www.instagram.com",
        "http://www.weibo.com",
        "http://www.sina.com.cn",
        "http://www.linkedin.com",
        "http://www.yahoo.co.jp",
        "http://www.msn.com",
        "http://www.vk.com",
        "http://www.google.de",
        "http://www.yandex.ru",
        "http://www.hao123.com",
        "http://www.google.co.uk",
        "http://www.reddit.com",
        "http://www.ebay.com",
        "http://www.google.fr",
        "http://www.t.co",
        "http://www.tmall.com",
        "http://www.google.com.br",
        "http://www.360.cn",
        "http://www.sohu.com",
        "http://www.amazon.co.jp",
        "http://www.pinterest.com",
        "http://www.netflix.com",
        "http://www.google.it",
        "http://www.google.ru",
        "http://www.microsoft.com",
        "http://www.google.es",
        "http://www.wordpress.com",
        "http://www.gmw.cn",
        "http://www.tumblr.com",
        "http://www.paypal.com",
        "http://www.blogspot.com",
        "http://www.imgur.com",
        "http://www.stackoverflow.com",
        "http://www.aliexpress.com",
        "http://www.naver.com",
        "http://www.ok.ru",
        "http://www.apple.com",
    ];

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

    for status in results {
        println!("{:?}", status);
    }

    println!("All threads have finished");
}
