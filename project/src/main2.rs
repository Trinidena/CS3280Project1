//1.	Accept, validate and parse the data folder name (with path) from the command line argument.
//2.	Check if the output folder exists in the data folder - if not, create it.  Assume that all the input folders will be there for all the branches.
//3.	Start a timer
//4.	Call the file input function process_input_file in lib.rs and pass the list of folders (with path) for all the branches.
//5.	Output to console the message received from the input function in lib.rs file
//6.	Stop the timer and show the total time in the console
//7.	Print the following message to indicate processing of all files are done - “Phew!  I am done.”
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

mod lib2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <parent_folder_path>", args[0]);
        return;
    }

    let parent_folder_path = &args[1];

    let subfolders = read_folders_from_parent_directory(parent_folder_path);
    let (sender, receiver) = mpsc::channel();

    let start_time = Instant::now();

    // Split subfolders into groups of 10
    let mut groups: Vec<Vec<String>> = Vec::new();
    for chunk in subfolders.chunks(10) {
        groups.push(chunk.to_vec());
    }

    let mut handles = vec![];
    for group in groups {
        let sender_clone = sender.clone();
        let handle = thread::spawn(move || {
            if let Err(err) = lib2::process_input_file(group, sender_clone) {
                println!("{}", err);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for message in receiver.iter() {
        println!("{}", message);
    }

    let elapsed_time = start_time.elapsed();
    println!(
        "Elapsed time: {}.{:03} seconds",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
    println!("Phew! I am done.");
}

fn read_folders_from_parent_directory(parent_folder_path: &str) -> Vec<String> {
    let mut folders = Vec::new();

    if let Ok(entries) = fs::read_dir(parent_folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let folder_path = entry.path();
                if folder_path.is_dir() {
                    if let Some(folder_path) = folder_path.to_str() {
                        println!("Path as String: {}", folder_path);
                        folders.push(folder_path.to_string());
                    } else {
                        println!("Invalid path");
                    }
                }
            }
        }
    }
    folders
}

fn check_for_output_folder(data_folder_path: &String) {
    let output_folder_name = "output";

    let output_folder_path = format!("{}/{}", data_folder_path, output_folder_name);

    if let Err(_) = fs::metadata(&output_folder_path) {
        match fs::create_dir(&output_folder_path) {
            Ok(_) => println!("Output folder created: {}", output_folder_path),
            Err(e) => {
                println!("Error: Failed to create output folder: {}", e);
                return;
            }
        }
    } else {
        println!("Output folder already exists: {}", output_folder_path);
    }
}

fn write_to_summary_file(output_string: &str) -> bool {
    let summary_file_path = "summary.txt";

    if let Ok(mut file) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(summary_file_path)
    {
        true
    } else {
        false
    }
}
