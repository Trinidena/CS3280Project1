// lib.rs

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::Sender;

pub fn process_input_file(folders: Vec<String>, sender: Sender<String>) -> Result<(), String> {
    for folder in &folders {
        let input_file_path = format!("{}/branch_weekly_sales.txt", folder);

        match fs::File::open(&input_file_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut total_quantity = 0;

                for line in reader.lines() {
                    if let Ok(line) = line {
                        let parts: Vec<&str> = line.trim().split(',').collect();
                        if parts.len() == 4 {
                            let quantity_sold: u32 = parts[2].trim().parse().unwrap_or(0);
                            total_quantity += quantity_sold;
                        } else {
                        }
                    } else {
                    }
                }

                let output_string = format!("{}, {}, {}", folder, "PROD001", total_quantity);
                if sender.send(output_string).is_err() {}
            }
            Err(_) => {
                if sender
                    .send(format!("ERROR: Failed to open file for folder {}", folder))
                    .is_err()
                {}
            }
        }
    }

    Ok(())
}
