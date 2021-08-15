use std::io;
use std::io::Write;

fn main() {
    let mut year = String::new();
    print!("Please input a year to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year", year);
    }

    let circle = Circle::small_circle();
    println!("Circle1's diameter: {}", circle.diameter());
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}

struct Circle {
    radius: u32,
}

impl Circle {
    // メソッド
    // 第一引数に self を渡す
    // circle.diameter で呼び出す
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    // 関連関数
    // Circle::small_circle で呼び出す
    fn small_circle() -> Circle {
        Circle { radius: 1 }
    }
}
