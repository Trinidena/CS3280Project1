use std::fs;
use std::io::{BufRead, BufReader, Write};

struct LineData {
    branch_code: String,
    product_code: String,
    quantity_sold: u32,
}

pub fn process_input_file(folders: Vec<String>) -> String {
    let mut result = String::new();
    let mut errors = Vec::new();

    for folder in folders {
        let input_file_path = format!("{}/branch_weekly_sales.txt", folder);

        if let Ok(file) = fs::File::open(&input_file_path) {
            let reader = BufReader::new(file);
            let mut total_quantity = 0;

            for (line_number, line) in reader.lines().enumerate() {
                let line = line.unwrap_or_else(|_| "".to_string());
                let parts: Vec<&str> = line.trim().split(',').collect();

                if parts.len() == 4 {
                    let quantity_sold: u32 = parts[2].trim().parse().unwrap_or(0);
                    total_quantity += quantity_sold;
                } else {
                    return "ERROR".to_string();
                }
            }

            let line_data = LineData {
                branch_code: folder.clone(),
                product_code: "PROD001".to_string(),
                quantity_sold: total_quantity,
            };

            let output_string = format!(
                "{}, {}, {}",
                line_data.branch_code, line_data.product_code, line_data.quantity_sold
            );

            if write_to_summary_file(&output_string) {
                result.push_str("OK\n");
            } else {
                errors.push(format!(
                    "Error writing to summary file for folder: {}",
                    folder
                ));
            }
        } else {
            errors.push(format!("Error opening input file for folder: {}", folder));
        }
    }
    result
}

fn write_to_summary_file(output_string: &str) -> bool {
    let summary_file_path = "summary.txt";

    if let Ok(mut file) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(summary_file_path)
    {
        if writeln!(file, "{}", output_string).is_ok() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

//         /Users/trinidaddena/Downloads/data
