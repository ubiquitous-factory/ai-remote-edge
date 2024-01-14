use dotenv::dotenv;
use serialport;
use std::env;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::process;

fn main() {
    dotenv().ok();

    let port_name = env::var("PORT_NAME").unwrap_or_else(|_| "/dev/ttyACM0".to_string());

    let baud_rate = env::var("BAUD_RATE")
        .unwrap_or_else(|_| "57600".to_string())
        .parse::<u32>()
        .unwrap();

    let line_count = env::var("LINE_COUNT")
        .unwrap_or_else(|_| "200".to_string())
        .parse::<u32>()
        .unwrap();

    let file_count = env::var("FILE_COUNT")
        .unwrap_or_else(|_| "200".to_string())
        .parse::<u32>()
        .unwrap();

    let mut rx = serialport::new(port_name, baud_rate)
        .timeout(std::time::Duration::from_secs(2))
        .open_native()
        .unwrap();

    let mut s = String::new();
    let mut current_file = 0;
    let mut current_line = 1;

    let file_name = format!("{:0>4}.csv", current_file);
    assert_eq!("0000", format!("{:0>4}", "0"));

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    loop {
        let mut buf = [0u8; 4098];

        match rx.read(&mut buf) {
            Ok(count) => {
                if current_file == file_count {
                    println!("Success 200 files created");
                    process::exit(0);
                }
                s.push_str(match std::str::from_utf8(&buf[..count]) {
                    Ok(val) => val,
                    Err(_) => {
                        println!("got non UTF-8 data from device");
                        ""
                    }
                });

                for line in s.lines() {
                    let values: Vec<&str> = line.split(';').collect();
                    // validate the line is complete
                    if values.len() == 4 && values[3].contains('F') {
                        // print out the celcius value with no C
                        let output = format!(
                            "{:?},{}\n",
                            chrono::offset::Utc::now(),
                            values[2].replace(" C", "")
                        );

                        print!("{}", output);
                        file.write(output.as_bytes()).expect("String didn't write");
                        if current_line == line_count {
                            current_file += 1;

                            let current_file_name = format!("{:0>4}.csv", current_file);
                            println!(
                                "Wrote {} lines in previous file. Changing file to {}.",
                                current_line, current_file_name
                            );
                            file = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .append(true)
                                .open(current_file_name)
                                .unwrap();
                            current_line = 0;
                        }
                        current_line += 1;
                    }
                }
                let has_a_line = s.rsplit_once('\n');

                if let Some(d) = has_a_line {
                    s = d.1.to_string();
                }
            }
            Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
