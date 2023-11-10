/// # Panics
///
/// Will panic if the two string slices do not have the same number of lines
pub fn stitch<'a>(left: &'a str, right: &'a str) -> String {
    assert_eq!(left.lines().count(), right.lines().count());

    let num_lines = right.lines().count();

    let my_iterator = |s: &'a str, n: usize| {
        s.lines().nth(n).unwrap()
    };

    let mut result = String::new();

    result.push_str(my_iterator(left, 0));
    result.push_str(my_iterator(right, 0));

    for n in 1..num_lines {
        result.push('\n');
        result.push_str(my_iterator(left, n));
        result.push_str(my_iterator(right, n));
    }

    result
}

#[cfg(test)]
mod tests {
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
}
