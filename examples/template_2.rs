use rust_hfdp::template_2::Duck;

fn main() {
    let mut ducks: Vec<Duck> = vec![
        Duck::new("Daffy", 8),
        Duck::new("Dewey", 2),
        Duck::new("Howard", 7),
        Duck::new("Louie", 2),
        Duck::new("Donald", 10),
        Duck::new("Huey", 2),
    ];

    println!("Before sorting...");
    display(ducks.clone());

    ducks.sort();
    println!("\nAfter sorting...");
    display(ducks.clone());
}

fn display(ducks: Vec<Duck>) {
    for duck in ducks {
        println!("{}", duck);
    }
}
