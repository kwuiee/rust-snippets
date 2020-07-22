use argh::FromArgs;

fn default_height() -> usize {
    5
}

#[derive(FromArgs)]
/// Reach new heights.
struct GoUp {
    /// an optional nickname for the pilot
    #[argh(option)]
    pilot_nickname: Option<String>,

    /// an optional height
    #[argh(option, default = "default_height()")]
    height: usize,

    /// an optional direction which is "up" by default
    #[argh(option, default = "String::from(\"only up\")")]
    direction: String,
}

fn main() {
    let up: GoUp = argh::from_env();
    println!(
        "name: {:?}, height: {}, direction: {}",
        up.pilot_nickname, up.height, up.direction
    );
}
