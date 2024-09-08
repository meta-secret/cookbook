mod utils;

use flume::{Receiver, Sender};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm!");
}

#[wasm_bindgen]
pub struct ReactiveState {
    sender: Sender<String>,
    receiver: Receiver<String>
}

#[wasm_bindgen]
impl ReactiveState {

    pub fn new() -> ReactiveState {
        let (sender, receiver) = flume::unbounded();
        ReactiveState { sender, receiver }
    }

    pub async fn send(&self, message: &str) {
        self.sender.send_async(message.to_string()).await.unwrap();
    }

    pub async fn receive(&self) -> String {
        self.receiver.recv_async().await.unwrap()
    }
}