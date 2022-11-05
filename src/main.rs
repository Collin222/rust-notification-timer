use notify_rust::Notification;
use std::env;
use std::io::stdin;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    // get message from args
    let message = parse_args();
    println!("Message : {message}");

    // get number of seconds of delay
    let res = get_user_input();
    if res.is_err() {
        let e = res.unwrap_err();
        match e {
            GetUserInputErrors::ReadLine => println!("Failed to read input"),
            GetUserInputErrors::Parse => println!("Invalid number of seconds provided"),
        }
        return ();
    }
    let num_seconds = res.unwrap();

    // send confirmation
    println!(
        "\nOk, sending notification every {} second{}",
        num_seconds,
        match num_seconds {
            1 => "",
            _ => "s",
        }
    );

    // create interval to send notification
    let mut interval = interval(Duration::from_secs(num_seconds));
    loop {
        interval.tick().await;
        create_notification(&message).show().unwrap();
        println!("Notification Sent");
    }
}

fn create_notification(message: &str) -> Notification {
    Notification::new().body(message).to_owned()
}

#[derive(Debug)]
enum GetUserInputErrors {
    ReadLine,
    Parse,
}

fn get_user_input() -> Result<u64, GetUserInputErrors> {
    let mut input = String::new();

    println!("How often do you want to send the notification? Enter a number of seconds.");
    let res = stdin().read_line(&mut input);

    if let Err(e) = res {
        println!("{e}");
        return Err(GetUserInputErrors::ReadLine);
    };

    match input.trim().parse() {
        Ok(v) => Ok(v),
        Err(_e) => Err(GetUserInputErrors::Parse),
    }
}

fn parse_args() -> String {
    let mut message = String::new();
    for arg in env::args().skip(1) {
        message.push_str(&arg);
        message.push(' ');
    }
    message
}
