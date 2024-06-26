// Port of https://www.rabbitmq.com/tutorials/tutorial-one-python.html. Start the
// hello_world_consume example in one shell, and run this in another.
use bnuuy::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(b"hello there", "hello"))?;

    connection.close()
}
