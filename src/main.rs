extern crate tokio;

use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::Parser;
#[cfg(unix)]
use rppal::gpio::Gpio;
use warp::Filter;
use warp::http::{Response, StatusCode};

mod morse;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The dit duration, in ms
    #[clap(short, long, default_value_t = 50)]
    duration: u32,

    // port that the webservice listens on
    #[clap(short, long, default_value_t = 80)]
    port: u16
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let gpio = Gpio::new()?;
    let led_pin = 14; // GPIO 14 == pin 8
    let mut pin = gpio.get(led_pin)?.into_output();
    pin.set_low();
    let pin = Arc::new(Mutex::new(pin));
    let dit_duration = args.duration;
    
    let encode = warp::post()
        .and(warp::path("encode"))
        .and(warp::body::form())
        .map(|m: HashMap<String, String>| {
            println!("got {m:?}");
            let text = match m.get("text") {
                Some(t) => morse::to_pretty(morse::encode(&t).unwrap()),
                None => "error".to_owned(),
            };
            text
        });
    
    let decode = warp::post()
        .and(warp::path("decode"))
        .and(warp::body::form())
        .map(|m: HashMap<String, String>| {
            println!("got {m:?}");
            let text: Result<_, Box<dyn Error>> = m.get("text").ok_or("missing field `text`".into());
            let deprettified: Result<_, Box<dyn Error>> = text.and_then(|p| morse::from_pretty(&p));
            let decoded = deprettified.and_then(|bits| {
                morse::decode(&bits)
            });
            match decoded {
                Ok(d) => Response::builder().body(d),
                Err(e) => Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(e.to_string()),
            }
        });
    
    let blink = warp::post()
        .and(warp::path("blink"))
        .and(warp::body::form())
        .map(move |m: HashMap<String, String>| {
            println!("got {m:?}");
            let mut pin = pin.lock().unwrap();
            let text = match m.get("text") {
                Some(t) => {
                    let enc = morse::encode(&t).unwrap();
                    let durations = morse::to_durations(enc);
                    for (pin_status, mut dur) in durations {
                        if pin_status {
                            pin.set_high();
                        } else {
                            pin.set_low();
                        }
                        dur /= 1000; // convert to ms
                        dur *= dit_duration; // set the dit duration in ms
                        thread::sleep(dur);
                    }
                    pin.set_low();
                },
                None => return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("no body to encode".into()),
            };
            Response::builder().body("success")
        });
    
    
    warp::serve(encode.or(decode).or(blink))
        .run(([0, 0, 0, 0], args.port))
        .await;
    Ok(())
}
