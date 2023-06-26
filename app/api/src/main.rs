use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

#[tokio::main]
async fn main() {
    let env_filter = Some(format!(
        "{}=DEBUG,tower_http=debug,axum::rejection=trace",
        env!("CARGO_PKG_NAME")
    ));
    utils::logger::init(env_filter);
    let smtp_host = "smtp.163.com";
    let smtp_port = 25;
    let smtp_user = "";
    let smtp_password = "";
    let from_user_name = "NoBody";
    let from = format!("{from_user_name} <{smtp_user}>").parse().unwrap();
    let to_user = "";
    let to_user_name = "Hei";
    let to = format!("{to_user_name} <{to_user}>").parse().unwrap();

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject("Happy new async year")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy with async!"))
        .unwrap();

    let creds = Credentials::new(smtp_user.to_owned(), smtp_password.to_owned());

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(smtp_host)
            .unwrap()
            .credentials(creds)
            .port(smtp_port)
            .build();

    match mailer.send(email).await {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
