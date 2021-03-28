mod channel_example;
mod state_example;
mod thread_example;

use channel_example::{basic_tx_rx, multiple_producers, multiple_tx};
use state_example::{basic_mutex, multithread_mutex};
use thread_example::{move_closure, new_thread, wait_thread};

fn main() {
    // Thread tutorial
    move_closure();
    new_thread();
    wait_thread();
    // Channel Example
    basic_tx_rx();
    multiple_tx();
    multiple_producers();
    // State Example
    basic_mutex();
    multithread_mutex();
}
