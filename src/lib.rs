use colored::Colorize;
use os_release::OsRelease;
use std::env;
use std::process::Command;

pub fn run() {
    // TODO colored data starts at the beginning and gets cut off...
    // how do I concat ColoredStrings together?
    let left = get_art();

    let user = env::var("USER")
        .expect("USER env var not working").blue();
    let hostname = env::var("HOSTNAME")
        .expect("HOSTNAME env var not working").blue();
    let os = OsRelease::new()
        .unwrap().pretty_name.white();

    let right: String = format!(" {user}@{hostname}\n {os}");

    println!("{}", stitch(&left, &right));
}

fn get_art() -> String {
    let output = Command::new("art_boxes")
        .arg("4")
        .arg("2")
        .output().ok().unwrap().stdout;

    String::from_utf8(output)
        .unwrap()
        .to_owned()
}

/// # Panics
///
/// Will panic if the two string slices do not have the same number of lines
fn stitch<'a>(left: &'a str, right: &'a str) -> String {
    assert_eq!(left.lines().count(), right.lines().count());

    let num_lines = right.lines().count();

    let my_iterator = |s: &'a str, n: usize| {
        s.lines().nth(n).unwrap()
    };

    let mut result = String::new();

    result.push_str(&left.lines().nth(0).unwrap().white().to_string());
    result.push_str(my_iterator(right, 0));

    for n in 1..num_lines {
        result.push('\n');
        result.push_str(&left.lines().nth(n).unwrap().white().to_string());
        result.push_str(my_iterator(right, n));
    }

    result
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn stitch_1() {
        let left  = "L\nL";
        let right = "R\nR";
        assert_eq!(stitch(left, right), "LR\nLR");
    }

    #[test]
    fn stitch_2() {
        let left  = "AB\nEF";
        let right = "CD\nGH";
        assert_eq!(stitch(left, right), "ABCD\nEFGH");
    }

    #[test]
    fn get_art_works() {
        get_art();
        assert!(true);
    }

    #[test]
    fn get_user_works() {
        let user = env::var("USER");
        assert!(user.is_ok())
    }

    #[test]
    fn get_hostname_works() {
        let hostname = env::var("HOSTNAME");
        assert!(hostname.is_ok())
    }

    #[test]
    fn get_os_works() {
        let _ = os_release::OsRelease::new()
            .unwrap().pretty_name;
        assert!(true);
    }
}
