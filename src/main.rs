use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};
use std::{thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}
pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn get_handler_action(&self) -> String {
        "user_created".to_string()
    }

    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let ten_millis = time::Duration::from_millis(1000);
        let now = time::Instant::now();
        thread::sleep(ten_millis);
        println!("In Syauqi’s Computer [2306275752]. Message received: {:?}", message);
        Ok(())
    }
}

fn main() {
    let listener =
        CrosstownBus::new_queue_listener("amqps://pweimtbc:2Z-u-gYWyK96CV5kojvTRbA9evBe0Qd4@gerbil.rmq.cloudamqp.com/pweimtbc".to_owned()
        ).unwrap();
    _ = listener.listen("user_created".to_owned(), UserCreatedHandler{},
                        crosstown_bus::QueueProperties { auto_delete: false, durable: false,
                            use_dead_letter: true });
    loop {
    }
}
