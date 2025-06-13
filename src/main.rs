use std::fs::File;
use std::path::Path;
use std::error::Error;
use rand::Rng;
use rand::seq::SliceRandom;
use sha2::{Sha256, Digest};
use std::io::BufWriter;

/// Generates a large CSV file with random data.
///
/// # Arguments
///
/// * `file_path` - The path to the output CSV file.
/// * `size_gb` - The desired file size in gigabytes.
/// * `names` - A slice of names to choose from randomly.
fn generate_large_csv(file_path: &str, size_gb: u64, names: &[&str]) -> Result<(), Box<dyn Error>> {
    let target_size_bytes = size_gb * 1024 * 1024 * 1024;
    let path = Path::new(file_path);

    println!("Starting to generate a {}GB CSV file at {}...", size_gb, file_path);
    println!("This process will take a significant amount of time and disk space.");

    // Create the file and wrap it in a BufWriter for performance.
    let file = File::create(&path)?;
    let mut writer = csv::Writer::from_writer(BufWriter::new(file));

    // Write the header record.
    writer.write_record(&["id", "name", "age"])?;

    let mut rng = rand::thread_rng();
    let mut row_count: u64 = 0;

    // Loop until the file size reaches the target.
    while path.metadata()?.len() < target_size_bytes {
        // Write in large batches to minimize I/O overhead.
        const BATCH_SIZE: usize = 10_000;
        for _ in 0..BATCH_SIZE {
            // --- FIX 1: Generate random bytes into a variable first. ---
            let random_bytes: [u8; 32] = rng.r#gen();
            let mut hasher = Sha256::new();
            hasher.update(random_bytes);
            let result = hasher.finalize();
            let id = hex::encode(result);

            // Choose a random name from the list.
            let name = *names.choose(&mut rng).unwrap_or(&"");

            // Generate a random age.
            let age = rng.gen_range(18..=60).to_string();

            // --- FIX 2: Pass all elements as references to create a slice of &str. ---
            writer.write_record(&[&id, name, &age])?;
            row_count += 1;
        }

        // Flush the buffer to disk to get an accurate file size.
        writer.flush()?;

        // Provide periodic progress updates.
        if row_count % 100_000 == 0 {
            let current_size_gb = path.metadata()?.len() as f64 / (1024.0 * 1024.0 * 1024.0);
            println!("Generated {} rows. Current file size: {:.2}GB", row_count, current_size_gb);
        }
    }

    let final_size_gb = path.metadata()?.len() as f64 / (1024.0 * 1024.0 * 1024.0);
    println!("\n--------------------------------------------------");
    println!("Successfully generated {}", file_path);
    println!("Total rows generated: {}", row_count);
    println!("Final file size: {:.2}GB", final_size_gb);
    println!("--------------------------------------------------");

    Ok(())
}

fn main() {
    // A list of common English short first names for data generation.
    let first_names = vec![
        "Liam", "Noah", "Jack", "Levi", "Owen", "John", "Leo", "Luke", "Ezra", "Luca",
        "Alex", "Alan", "Ben", "Kyle", "Kurt", "Lou", "Matt", "Ryan", "Mia", "Elias",
        "Mila", "Nova", "Axel", "Leon", "Amara", "Finn", "Molly", "Brian", "Dante",
        "Rhys", "Thea", "Otis", "Rohan", "Anne", "Britt", "Brooks", "Cash", "Dane",
        "Eve", "Gem", "Huck", "Ivy", "Lael", "Mack", "Maeve", "Nell", "Onyx", "Pace",
        "Quinn", "Reed", "Scout", "Taft", "Ula", "Van", "Wade", "West"
    ];

    // Define the output file path and the desired size in gigabytes.
    let output_file_path = "large_data_rust.csv";
    let desired_size_gb = 100;

    if let Err(e) = generate_large_csv(output_file_path, desired_size_gb, &first_names) {
        eprintln!("An error occurred: {}", e);
    }
}
