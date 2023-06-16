fn worker() -> Result<(), String> {
    let mut args = std::env::args();
    args.next(); // Skip program path
    let delay = args.next().ok_or_else(|| "expected 2 arguments, found 0".to_string())?
        .parse::<u64>().map_err(|_| "expected a positive number as the first argument".to_string())?;
    let url = args.next().ok_or_else(|| "expected 2 arguments, found 1".to_string())?;

    let client = reqwest::blocking::Client::new();

    let request = client.get(&url).build().map_err(|_| "URL parsing error".to_string())?;

    loop {
        print!("Checking '{}'. ", &url);
        
        let response = if let Ok(resp) = client.execute(request.try_clone().unwrap()) {
            resp
        } else {
            println!("Network error");
            continue;
        };

        match response.error_for_status() {
            Ok(_) => {
                println!("Result: OK(200)")
            }
            Err(err) => {
                println!("Result: ERR({})", err.status().unwrap().as_u16())
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(delay));
    }

    return Ok(());
}


fn main() {
    let result = worker();
    result.unwrap_or_else(|message| {
        println!("{}", message);
    });

    /*match worker() {
        Ok(()) => {}
        Err(err) if err.is_builder() => {
            println!("URL parsing error")
        }
        Err(err) if err.is_status() => {
            println!("error status code {}", err.status().as_u16())
        }
        Err(_) => {
            println!("other error")
        }
    }
    */
}
