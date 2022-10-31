use clap::Parser;
use lapin::{Result, Connection, ConnectionProperties, options::{BasicPublishOptions}, BasicProperties};
use model::User;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value="amqp://127.0.0.1:5672/%2f")]
    ampq_addr: String,
    #[arg(short, long, default_value="myqueue")]
    queue_name: String,
    user_name: String,
    email: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let user = User { user_name: args.user_name.clone(), email: args.email.clone() };

    async_global_executor::block_on(async {
        let conn = Connection::connect(
            &args.ampq_addr, ConnectionProperties::default(),
        ).await?;

        let channel = conn.create_channel().await?;

        let payload = User::serialize(&user).unwrap();

        let _ = channel
            .basic_publish(
                "",
                &args.queue_name,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;

        Ok(())
    })
}
