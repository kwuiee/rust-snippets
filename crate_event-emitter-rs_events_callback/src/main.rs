use event_emitter_rs::EventEmitter;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

// Using a more advanced value type such as a struct by implementing the serde traits
#[derive(Serialize, Deserialize)]
struct Date {
    month: String,
    day: String,
}

fn main() {
    let mut event_emitter = EventEmitter::new();

    event_emitter.on("Add three", |number: f32| println!("{}", number + 3.0));
    event_emitter.emit("Add three", 5.0 as f32);
    event_emitter.emit("Add three", 4.0 as f32);
    // >> "8.0"
    // >> "7.0"

    event_emitter.on("LOG_DATE", |date: Date| {
        println!("Month: {} - Day: {}", date.month, date.day)
    });
    event_emitter.emit(
        "LOG_DATE",
        Date {
            month: "January".to_string(),
            day: "Tuesday".to_string(),
        },
    );
    // >> "Month: January - Day: Tuesday"

    thread::sleep(Duration::from_secs(1));
}
