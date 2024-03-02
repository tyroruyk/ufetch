use std::collections::HashSet;
use std::process::{Command, exit};
use std::io::{self, BufRead};

fn main() {
    match get_gpu_model("on", "", "dedicated") {
        Ok(()) => println!("GPU retrieval successful."),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn trim(s: &str) -> &str {
    s.trim()
}

fn prin<S: AsRef<str>>(_subtitle: S, value: &str) {
    // Assuming that the prin function is meant to print values.
    // You can replace this with your desired output mechanism.
    println!("{}", value);
}

pub fn get_gpu_model(gpu_brand: &str, subtitle: &str, gpu_type: &str) -> io::Result<()> {
    // Run lspci -mm and capture its output
    let output = Command::new("lspci").arg("-mm").output();

    // Check if the command was successful
    match output {
        Ok(output) => {
            if output.status.success() {
                // Convert the output to a string
                let lspci_output = String::from_utf8_lossy(&output.stdout);

                // Read GPUs into HashSet
                let mut gpus = HashSet::new();
                for line in lspci_output.lines() {
                    if line.contains("Display") || line.contains("3D") || line.contains("VGA") {
                        let gpu_parts: Vec<&str> = line.split('"').collect();
                        let mut gpu_entry = gpu_parts[1].to_string();
                        gpu_entry.push(' ');
                        gpu_entry.push_str(gpu_parts[3]);
                        if gpu_entry.contains("Device ") {
                            gpu_entry.push(' ');
                            gpu_entry.push_str(gpu_parts[gpu_parts.len() - 2]);
                        }
                        gpus.insert(gpu_entry);
                    }
                }

                // Remove duplicate Intel Graphics outputs
                if let Some(first_gpu) = gpus.iter().next() {
                    if first_gpu.contains("Intel") {
                        gpus.remove(first_gpu);
                    }
                }

                for gpu in gpus {
                    let gpu_type_str = if gpu_type == "dedicated" && gpu.contains("Intel") {
                        continue;
                    } else if gpu_type == "integrated" && !gpu.contains("Intel") {
                        continue;
                    } else {
                        ""
                    };

                    let brand = match gpu {
                        _ if gpu.contains("AMD") || gpu.contains("ATI") => {
                            let brand = gpu.replace("Advanced Micro Devices, Inc.", "");
                            brand.replace("[AMD/ATI] ", "").replace("[AMD] ", "").replace("OEM ", "")
                        }
                        _ if gpu.contains("NVIDIA") => {
                            gpu.replace("[", "").replace("]", "").to_string()
                        }
                        _ if gpu.contains("Intel") => {
                            gpu.replace("(R)", "").replace("Corporation", "")
                                .replace(" (Xeon)", "").replace(" Integrated Graphics Controller", "")
                                .replace("Xeon", "Intel HD Graphics")
                        }
                        _ if gpu.contains("MCST") => {
                            gpu.replace("MCST MGA2", "")
                        }
                        _ if gpu.contains("VirtualBox") => {
                            "VirtualBox Graphics Adapter".to_string()
                        }
                        _ => {
                            continue;
                        }
                    };

                    let gpu_name = match gpu_brand {
                        "off" => {
                            gpu.replace("AMD ", "").replace("NVIDIA ", "").replace("Intel ", "")
                        }
                        _ => {
                            gpu.to_string()
                        }
                    };

                    prin(format!("{}{}", subtitle, gpu_name), &format!("{} {}", gpu_type_str, brand));
                }

                return Ok(());
            } else {
                // Print the error code and exit the program
                eprintln!("Command failed with error code: {}", output.status);
                exit(1);
            }
        }
        Err(error) => {
            // Print the error and exit the program
            eprintln!("Failed to execute lspci command: {}", error);
            exit(1);
        }
    }
}
