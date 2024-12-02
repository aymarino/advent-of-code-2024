mod day01;
mod day02;

fn run<FnT: Fn(&str) -> u32>(label: &str, func: FnT, filename: &str) {
    let input = std::fs::read_to_string(format!("inputs/{filename}")).unwrap();
    let result = func(&input);
    println!("{label}: {result}");
}

fn main() {
    run("1.1", day01::p1, "1.txt");
    run("1.2", day01::p2, "1.txt");
    run("2.1", day02::p1, "2.txt");
    run("2.2", day02::p2, "2.txt");
}
