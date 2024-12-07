mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn run<AccumT: std::fmt::Display, FnT: Fn(&str) -> AccumT>(label: &str, func: FnT, filename: &str) {
    let input = std::fs::read_to_string(format!("inputs/{filename}")).unwrap();
    let result = func(&input);
    println!("{label}: {result}");
}

fn main() {
    run("1.1", day01::p1, "1.txt");
    run("1.2", day01::p2, "1.txt");
    run("2.1", day02::p1, "2.txt");
    run("2.2", day02::p2, "2.txt");
    run("3.1", day03::p1, "3.txt");
    run("3.2", day03::p2, "3.txt");
    run("4.1", day04::p1, "4.txt");
    run("4.2", day04::p2, "4.txt");
    run("5.1", day05::p1, "5.txt");
    run("5.2", day05::p2, "5.txt");
    run("6.1", day06::p1, "6.txt");
    run("6.2", day06::p2, "6.txt");
    run("7.1", day07::p1, "7.txt");
    run("7.2", day07::p2, "7.txt");
}
