use types::BrokerExecutor;

pub mod types;

pub struct Traders {
    broker_executor: Box<dyn BrokerExecutor>,
}

impl Traders {
    pub fn register_broker_executor(&mut self, broker_executor: Box<dyn BrokerExecutor>) {
        self.broker_executor = broker_executor;
    }

    pub fn register_data_handler(&mut self) {}

    pub fn register_portfolio(&mut self) {}

    pub fn register_strategy(&mut self) {}

    pub async fn run(&self) {
        loop {
            // Get an event from event bus
            // Handle event
            // if ORDER event
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
