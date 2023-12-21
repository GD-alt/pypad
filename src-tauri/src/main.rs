// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;

#[tauri::command]
fn fetch_sys_py_interpreters() -> String {
    // Get PATH environment variable
    let path = std::env::var("PATH").unwrap_or_default();
    let paths = path.split(";");

    let mut sys_py_interpreters = Vec::new();

    for path in paths {
        // If path contains matches Python3{8 9 10 11}, add it to the list (regex)
        if regex::Regex::new(r"Python3[8-9]|Python31[0-1]").unwrap().is_match(path) && !path.contains("Scripts") {
            sys_py_interpreters.push(path);
        }
    }

    // Return list of python interpreters
    sys_py_interpreters.join("\n")
}

#[tauri::command]
fn run_code(code: String, interpreter: String) -> Vec<String> {
    // Create temporary file
    let mut file = tempfile::NamedTempFile::new().unwrap();

    // Write code to file
    file.write_all(code.as_bytes()).unwrap();

    // If interpreter path not ends with .exe, add it
    let interpreter = if !interpreter.ends_with(".exe") {
        interpreter + "python.exe"
    } else {
        interpreter
    };

    // Run code
    let output = std::process::Command::new(interpreter)
        .arg(file.path())
        .output()
        .expect("failed to execute process");

    // Return output in vec (stdout, stderr)
    vec![String::from_utf8(output.stdout).unwrap(), String::from_utf8(output.stderr).unwrap()]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_sys_py_interpreters,
            run_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
