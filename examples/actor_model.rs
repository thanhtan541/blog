use tokio::{sync::{oneshot, mpsc}, net::TcpStream, io::{self, AsyncWriteExt, AsyncReadExt}};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

async fn run_my_actor(mut act: MyActor) {
    while let Some(msg) = act.receiver.recv().await {
        act.handle_message(msg).await.unwrap();
    }
}

// What is actor
// An actor is a background task that uses message passing
// to communicate with the rest of the program
// Usercase of Actor
// For IO resources that we need to have shared-access.
struct MyActor {
    // FIFO queue for actor, aka: actor's address that other actor want to communicate
    receiver: mpsc::Receiver<ActorMessage>,
    // ip resource
    connection: TcpStream,
}

enum ActorMessage {
    SendMessage {
        // String is used to avoid complexity of handling &str which
        // involve lifetime annotation
        message: String,
        // Next hope actor address - usually the sennder
        response_to: oneshot::Sender<u32>
    }
}

impl MyActor {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>, connection: TcpStream) -> Self {
        Self {receiver, connection}
    }

    async fn handle_message(&mut self, msg: ActorMessage) -> io::Result<()> {
        match msg {
            ActorMessage::SendMessage { message, response_to } => {
                self.connection.write_all(message.as_bytes()).await?;
                let response = self.connection.read_u32().await?;
                let _ = response_to.send(response);
                Ok(())
            }
        }
    }
}

// A handler to setup actor
struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>
}

impl MyActorHandle {
    // Spin up the actor instance 
    pub fn new(connection: TcpStream) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = MyActor::new(receiver, connection);
        tokio::spawn(run_my_actor(actor));

        Self {sender}
    }
}

//Caveats:
//-- Backpress
//-- Deadlock
//-- Memory leak
