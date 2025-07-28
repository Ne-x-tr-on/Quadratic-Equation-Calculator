use std::io;
use std::f64;

fn main() {
    println!("🧮 Quadratic Equation Calculator: ax² + bx + c = 0");

    let a = get_input("Enter coefficient a: ");
    let b = get_input("Enter coefficient b: ");
    let c = get_input("Enter coefficient c: ");

    if a == 0.0 {
        println!("This is not a quadratic equation (a cannot be 0).");
        return;
    }

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("✅ Two real roots: x₁ = {:.2}, x₂ = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("✅ One real root: x = {:.2}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!(
            "✅ Two complex roots: x₁ = {:.2} + {:.2}i, x₂ = {:.2} - {:.2}i",
            real_part, imaginary_part, real_part, imaginary_part
        );
    }
}

fn get_input(prompt: &str) -> f64 {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("❌ Please enter a valid number."),
        }
    }
}
