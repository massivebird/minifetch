use colored::{Color, Colorize};
use os_release::OsRelease;
use std::{env, process::Command};

fn main() {
    let left = art_streets::generate_art(4, 2);

    let user = env::var("USER")
        .unwrap_or_else(|_| "<missing user>".to_string())
        .blue();

    let symbol = "@".bright_white();

    let hostname = env::var("HOSTNAME")
        .or_else(|_| {
            Command::new("hostname")
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .ok_or(env::VarError::NotPresent)
        })
        .unwrap_or_else(|_| "<missing hostname>".to_string())
        .trim_end()
        .blue();

    let os = OsRelease::new().unwrap().pretty_name.white();

    let right: String = format!(" {user}{symbol}{hostname}\n {os}");

    println!("{}", stitch(&left, &right, None));
}

/// Prints two strings horizontally adjacent to each other.
///
/// Preferably, both input strings are of equal length. Otherwise, I have no idea what will happen.
fn stitch<'a>(left: &'a str, right: &'a str, left_color: Option<Color>) -> String {
    let get_nth_line = |s: &'a str, n: usize| {
        left_color.map_or_else(
            || String::from(s.lines().nth(n).unwrap()),
            |c| s.lines().nth(n).unwrap().color(c).to_string(),
        )
    };

    let mut buf = String::new();
    buf.push_str(&get_nth_line(left, 0));
    buf.push_str(&get_nth_line(right, 0));

    for n in 1..left.lines().count() {
        buf.push('\n');
        buf.push_str(&get_nth_line(left, n));
        buf.push_str(&get_nth_line(right, n));
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::stitch;

    #[test]
    fn stitch_works() {
        let left = "AB\nEF";
        let right = "CD\nGH";
        assert_eq!(stitch(left, right, None), "ABCD\nEFGH");
    }
}
