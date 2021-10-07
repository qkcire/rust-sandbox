pub fn run() {
    // Print to console
    println!("Hello World from the print.rs file");

    // Basic formatting
    println!("Number: {}", 1);

    println!("{} is from {}.", "Erick", "Cal");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}.", "Erick", "Cal", "code");

    // Named arguments
    println!("{name} likes to {activity}.", name="Erick", activity="play guitar");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}