mod transmitter;

use transmitter::{
    TransmitJob,
};

use rustyline::{
    error::{
        ReadlineError,
    },
    Editor,
};

use tokio::{
    sync::Mutex,
    task,
};

use sysfs_gpio::{
    Pin,
};

use std::sync::{
    Arc,
};

/*
    TODO: Implement Signal-Hook for a graceful exit and writing logs to a file
        using the instructions and examples given here: https://rust-cli.github.io/book/in-depth/signals.html
*/

#[tokio::main]
async fn main() {
    // Instantiate the runtime
    // Mutex to lock access to gpio pin
    // Use the gpio<number> pins from the header readout
    let gpio_pin = Arc::new(Mutex::new(Pin::new(445)));
    println!("Pin created");
    // REPL loop crate initialised
    let mut rl = Editor::<()>::new();
    println!("REPL initialised");
    // Vector of JoinHandles, will be used to keep track of all active transmissions
    let mut job_queue = Vec::new();
    println!("Job Queue initialised");
    if rl.load_history("history.txt").is_err() {
        println!("No previous history");
    }

    loop {
        // defining a prompt for rusty line to strip from every read line command
        let read_line = rl.readline("> ");

        // Making a clone of the ARC for local use
        let gpio_clone = Arc::clone(&gpio_pin);

        match read_line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut command = line.split(' ').collect::<Vec<_>>();
                if command.len() != 2 {
                    command.push("1");
                }
                let mut current_job = TransmitJob::new(
                    command[0].to_string().clone(),
                    command[1].parse::<u8>().unwrap_or(1),
                    gpio_clone,
                );

                // The latest message in the transmit queue is sent off to be transmitted
                // Spawns a thread everytime a new job is acquired
                let job = task::spawn(async move {
                    current_job.transmit().await
                });
                job_queue.push(job);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL + C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL + D");
            },
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}