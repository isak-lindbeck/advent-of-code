pub fn run(input: String) -> (usize, usize) {
    let input_replaced = input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    let ans_1: usize = input.lines().map(to_num).sum();
    let ans_2: usize = input_replaced.lines().map(to_num).sum();
    (ans_1, ans_2)
}

fn to_num(line: &str) -> usize {
    let x = line.chars().find(char::is_ascii_digit).unwrap();
    let y = line.chars().rfind(char::is_ascii_digit).unwrap();
    format!("{x}{y}").parse().unwrap()
}
