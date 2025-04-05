use crate::hyaena::utils::FilePathInfo;
use crate::hyaena::ReadOutput;
use rand::seq::SliceRandom; // For shuffling
use rand::thread_rng; // For random number generator
use std::fs; // For file writing

pub fn soup() {
    println!("Starting cook process...\n");

    println!("Getting the ingredients...");

    let read_data: ReadOutput = crate::hyaena::new();

    // Ensure we have content to process
    if let Some(content) = read_data.content {
        println!("Read {} bytes from input file.", content.len());

        // 1. Split content into words
        let mut words: Vec<String> = content
            .split_whitespace() // Splits by any whitespace
            .map(|s| s.to_lowercase()) // Convert &str slices to owned Strings and lowercase
            .collect();
        println!("Found {} words.", words.len());

        if words.is_empty() {
            println!("Input file contains no words to process.");
            return;
        }

        // 2. Shuffle the words randomly
        let mut rng = thread_rng();
        words.shuffle(&mut rng);
        println!("Shuffled words.");

        // 3. Group words into lines of 5
        println!(
            "Grouping words into lines of {} words.",
            read_data.config.words_per_line
        );
        let lines: Vec<String> = words
            .chunks(read_data.config.words_per_line) // Create slices of up to 5 words
            .map(|chunk| chunk.join(" ")) // Join words in the chunk with spaces
            .collect();
        println!("Grouped words into {} lines.", lines.len());

        // 4. Join lines with newline characters
        let mut output_content = lines.join("\n");
        output_content.push('\n'); // Add a final newline for good measure

        // 5. Define output filename (you might want to make this configurable later)

        let output_filename = FilePathInfo::new(
            &format!("{}", &read_data.config.input_filename),
            "output_randomized",
            Some("md".to_string()),
        )
        .output_file_path;

        // 6. Write the randomized content to the new file
        match fs::write(&output_filename, output_content) {
            Ok(_) => println!(
                "✅ Successfully wrote randomized words to output file: '{}'",
                output_filename
            ),
            Err(e) => eprintln!("Error writing to output file '{}': {}", output_filename, e),
        }
    } else {
        println!(
            "❌ Could not read content from '{}'. Is it a valid .md file?",
            read_data.config.input_filename
        );
    }
}
