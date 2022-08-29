use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

#[derive(Message)]
#[rtype(result = "usize")]
struct Greet(String);

struct MyActor {
    count: usize,
    message: String
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for MyActor {
    type Result = usize;
    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

impl Handler<Greet> for MyActor {
    type Result = usize;
    fn handle(&mut self, msg: Greet, ctx: &mut Self::Context) -> Self::Result {
        self.message = msg.0;
        self.count += self.message.len();
        println!("Sender says: {}!", &self.message);
        self.count
    }
}

#[actix_rt::main]
async fn main() {
    let addr = MyActor { count: 10, message: "".to_owned() }.start();
    let mut res = addr.send(Ping(10)).await;
    res = addr.send(Greet("Hello, Actix!".to_owned())).await; 
    println!("New size: {}", res.unwrap());
    System::current().stop();
}
