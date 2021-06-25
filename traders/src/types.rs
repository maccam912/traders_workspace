enum Event {
    MARKET,
    SIGNAL,
    ORDER,
    FILL,
}

pub trait BrokerExecutor {
    fn send_order(&self, order: String);
}
