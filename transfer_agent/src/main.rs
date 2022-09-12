use actix_web::{ post, Responder, HttpServer, App, HttpResponse };

mod resolver;

use resolver::{ IpAddress, TransferServer };

#[post("/send-mail")]
async fn send_mail(to: String) -> impl Responder {
    let message = format!("Trying to send message to {}", to);
    println!("{}", message);
    HttpResponse::Ok().body(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut transfer_agent = TransferServer::new(IpAddress::new(127, 0, 0, 1), "localhost");
    HttpServer::new(|| {
        App::new()
            .service(send_mail)
    })
    .bind(("127.0.0.1", 26535))?
    .run()
    .await
}
