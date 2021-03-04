use std::{
    fmt::{
        Display,
        Formatter,
        Error
    },
    sync::Arc,
    future::Future,
    task::{
        Context,
        Poll
    },
};

use sysfs_gpio::{
    Direction,
    Pin,
};

use tokio::{
    time::{
        sleep,
        Duration,
    },
    sync::Mutex,
};

pub struct TransmitJob {
    string_transmission: String,
    binary_transmission: Option<String>,
    times: u8,
    pin: Arc<Mutex<Pin>>,
    transmitting: bool,
    completed_transmission: bool,
}

impl TransmitJob {
    pub fn new(transmission: String, number: u8, pin: Arc<Mutex<Pin>>) -> TransmitJob {
        TransmitJob {
            string_transmission: transmission.clone(),
            binary_transmission: TransmitJob::convert_to_binary(transmission),
            times: number,
            pin,
            transmitting: false,
            completed_transmission: false,
        }
    }

    /*
        Converts a Rust String into a binary version of the
        same effective value. Defaulting to 8 bit codepoints.
        forcing ASCII for now.
     */
    fn convert_to_binary(input: String) -> Option<String> {
        if input.is_empty() || !input.is_ascii() {
            return None::<String>;
        }
        let mut string_in_binary = String::new();

        for character in input.clone().into_bytes() {
            string_in_binary += &format!("0{:b}", character);
        }

        Some(string_in_binary)
    }

    /*
        Checks if it's a valid transmission then performs an asynchronous transmission of the job
        Returns the JoinHandle to the calling program to keep the task running outside the function
     */
    pub async fn transmit(&mut self) -> sysfs_gpio::Result<()> {
        let pin = self.pin.lock().await.to_owned();
        self.transmitting = true;
        println!("Transmitting: \n{}", self);
        // Export Pin
        pin.export().unwrap();
        // Set Direction, define as pin_out
        pin.set_direction(Direction::Out).unwrap();
        // Need to sleep for at least 80ms before can start polling
        sleep(Duration::from_millis(100)).await;
        // Iterate through the binary transmission
        for _ in 0..self.times {
            for character in self.binary_transmission.clone().unwrap().chars() {
                TransmitJob::gpio_actual(character, pin).await;
            }
        }
        self.completed_transmission = true;
        Ok(())
    }

    async fn gpio_actual(transmission_bit: char, pin: Pin) {
        // parse into base2
        let bit = transmission_bit.to_digit(2).unwrap();
        // Set the value
        pin.set_value(bit as u8).unwrap();
        // Sleep put the thread to sleep for 33.333 ms
        // An effective 30Hz polling rate
        sleep(Duration::from_millis(33.333 as u64)).await;
    }
}

impl Future for TransmitJob {
    type Output = sysfs_gpio::Result<()>;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed_transmission {
            Poll::Ready(Ok(()))
        }
        else if !self.transmitting {
            Poll::Pending
        }
        else {
            Poll::Pending
        }
    }
}

impl Display for TransmitJob {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.string_transmission.is_empty() {
            write!(f, "Job is empty.")
        } else if self.binary_transmission.is_none() {
            write!(f, "Cannot be converted to a binary transmission.")
        }
        else {
            write!(f, "String: {}\nBits: {}\nTimes: {}",
                   self.string_transmission,
                   self.binary_transmission.as_ref()
                       .unwrap_or(&"Not an ASCII String".to_string()),
                   self.times)
        }
    }
}
