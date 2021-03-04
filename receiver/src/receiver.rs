#[allow(unused_imports, dead_code, unused_mut)]

use std::{
    str,
    fmt::{
        Display,
        Formatter,
        Error
    },
    task::{
        Context,
        Poll
    },
};

use futures::Future;

pub enum ReceiveState {
    StartCommand,
    EndCommand,
}

pub struct ReceiveJob<'a> {
    input_vec: & 'a Box<Vec<u8>>,
    string_transmission: String,
    start: usize,
    analysing: bool,
    completed_analysing: bool,
}

impl ReceiveJob <'_> {
    // dispatched when a transmission is started and ended
    pub fn new(input_vec: &Box<Vec<u8>>, start: usize) -> ReceiveJob {
        ReceiveJob{
            input_vec,
            string_transmission: String::new(),
            start,
            analysing: false,
            completed_analysing: false
        }
    }

    pub fn convert_to_ascii(binary_input: String) -> Option<String> {
        let vec_string = (0..binary_input.len())
            .step_by(9)
            .map(|i| u8::from_str_radix(&binary_input[i..i + 8], 2).unwrap()).collect();

        match String::from_utf8(vec_string) {
            Ok(s) => Some(s),
            Err(e) => None,
        }
    }

    pub async fn analyse(&self) -> Option<String> {
        let mut prev_size: usize = self.input_vec.len();
        loop {

        }
    }
}

impl Future for ReceiveJob <'_> {
    type Output = sysfs_gpio::Result<()>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed_analysing{
            Poll::Ready(Ok(()))
        }
        else if !self.analysing {
            Poll::Pending
        }
        else {
            Poll::Pending
        }
    }
}


impl Display for ReceiveJob <'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.string_transmission.is_empty() {
            write!(f, "Job is empty")
        } else if self.input_vec.is_empty() {
            write!(f, "Cannot be converted to an ascii message")
        } else {
            write!(f, "Bits: {}\nASCII: {}",
                   self.string_transmission,
                   str::from_utf8(self.input_vec).unwrap(),
            )
        }
    }
}

pub async fn analyse_transmission_state(bit_buffer: &[u8]) -> Option<ReceiveState> {
    if bit_buffer.iter().all(|&b| b == 0) {
        Some(ReceiveState::EndCommand)
    }
    else if bit_buffer.iter().all(|&b| b == 1) {
        Some(ReceiveState::StartCommand)
    }
    else {
        None
    }
}