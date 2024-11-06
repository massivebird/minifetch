use colored::{Color, Colorize};
use os_release::OsRelease;
use std::env;
use std::process::Command;

fn main() {
    let left = get_art();

    let user = env::var("USER")
        .unwrap_or_else(|_| "defaultuser".to_string())
        .blue();

    let symbol = "@".bright_white();

    let hostname = env::var("HOSTNAME")
        .or_else(|_| String::from_utf8(Command::new("hostname").output().unwrap().stdout))
        .unwrap_or_else(|_| String::from("<missing hostname>"))
        .trim_end()
        .blue();

    let os = OsRelease::new().unwrap().pretty_name.white();

    let right: String = format!(" {user}{symbol}{hostname}\n {os}");

    println!("{}", stitch(&left, &right, None));
}

fn get_art() -> String {
    let output = Command::new("art_boxes").arg("4").arg("2").output().ok();

    match output {
        Some(x) => String::from_utf8(x.stdout).unwrap(),
        None => String::from("\n\n"),
    }
}

/// Prints two strings horizontally adjacent to each other.
///
/// # Panics
///
/// Will panic if the two string slices do not have the same number of lines
fn stitch<'a>(left: &'a str, right: &'a str, left_color: Option<Color>) -> String {
    assert_eq!(left.lines().count(), right.lines().count());

    let get_nth_line = |s: &'a str, n: usize| {
        left_color.map_or_else(
            || String::from(s.lines().nth(n).unwrap()),
            |c| s.lines().nth(n).unwrap().color(c).to_string(),
        )
    };

    let mut result = String::new();
    result.push_str(&get_nth_line(left, 0));
    result.push_str(&get_nth_line(right, 0));

    for n in 1..left.lines().count() {
        result.push('\n');
        result.push_str(&get_nth_line(left, n));
        result.push_str(&get_nth_line(right, n));
    }

    result
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
