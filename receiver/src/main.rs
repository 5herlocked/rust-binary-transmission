#[allow(unused_imports, dead_code)]
mod receiver;

use receiver::{
    ReceiveJob,
    ReceiveState,
    analyse_transmission_state,
};

use rayon::prelude::*;

use sysfs_gpio::{
    Pin,
    Direction
};

use std::{
    sync::{
        Arc,
        Mutex,
    },
    time::Duration,
    thread::sleep
};

fn main() {
    /*
        TODO: A continuous write stream that converts to text from binary after printing the binary
            01010101 01010101 01010101 01010101 01010101
            -> UUUUU

        We need 2 buffers ->
            1st is a continues vector read in from the pin
     */

    let _process = async {
        let gpio_pin = Arc::new(Mutex::new(Pin::new(445)));
        // Pin exported
        gpio_pin.lock().unwrap().export().unwrap();
        // Direction set, defined as pin_in
        gpio_pin.lock().unwrap().set_direction(Direction::In).unwrap();
        // Sleep for at least 80ms before polling
        sleep(Duration::from_millis(100));
        // announce that pin is ready
        println!("Pin created");

        let mut receiver_stream: Box<Vec<u8>> = Box::new(Vec::new());

        //let _unused_analyser = rayon::spawn(async { analyse(receiver_stream).await });

        // This is just the data ingest loop
        loop {
            let gpio_clone = gpio_pin.clone();
            receiver_stream.push(gpio_clone.lock().unwrap().get_value().unwrap());
            // 16.666 ms polling effective 60Hz
            sleep(Duration::from_millis(16.666 as u64));
        }
    };
}

// async fn analyse(stream: Box<Vec<u8>>) {
//     let mut tracker: usize = 0;
//     let mut transmission_state: Option<ReceiveState>;
//     loop {
//         // check length
//         // read until ReceiveState::START is found
//         // then capture the transmission until ReceiveState::END is found
//         let tracker_clone = tracker.clone();
//         if stream.len() >= 15 {
//             transmission_state = match analyse_transmission_state(&stream[tracker..tracker+15]).await {
//                 Some(ReceiveState::StartCommand) => {
//                     // This is where the transmission has started
//                     // Create a new ReceiveJob?
//                     rayon::spawn( || {
//                         let job = ReceiveJob::new(&stream, tracker_clone);
//                         job.analyse();
//                     });
//
//                     Some(ReceiveState::StartCommand)
//                 }
//                 None => {
//                     // This is just a continue statement
//                     None
//                 }
//                 _ => None,
//             };
//         }
//     }
// }
