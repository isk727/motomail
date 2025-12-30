use chrono::Local;
use reqwest::Client;
use serde::Deserialize;
use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use lettre::{
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
    message::{Mailbox, SinglePart, header::ContentType},
};

struct Config {
    api_url: String,
    api_key: String,
    server_name: String,
    app_user: String,
    app_pass: String,
    sender_name: String,
    sender_mail: String,
    log_file: String,
}

# [derive(Debug, Deserialize)]
struct MotoMail {
    ver: String,
    to: String,
    cc: String,
    subject: String,
    body: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
# [tokio::main]
async fn main() -> Result<()> {
    let config = Config {
        api_url: String::from("https://www.motorolahis.jp/mothis/jsp/mail/api/api-mail.jsp"),
        api_key: String::from("Ym93cvPYQ3Fp7o"),
        server_name: String::from("smtp.gmail.com"),
        app_user: String::from("mothis49@gmail.com"),
        app_pass: String::from("ogkpbdccxqmpvcit"),
        sender_name: String::from("モトローラ健康保険組合"),
        sender_mail: String::from("res2011@freescale.com"),
        log_file: String::from("/var/log/motomail/motomail.log"),
    };
    // --------------------------------------------------------------
    let client = Client::new();
    let response = client
        .get(config.api_url)
        .query(&[("api-key", config.api_key)])
        .send()
        .await?;
    let mail = response.json::<MotoMail>().await?;
    // --------------------------------------------------------------
    let subject = mail.subject;
    if mail.to == "NULL" {
        println!("{}", mail.to);
    } else {
        let sender_mailbox = Mailbox::new(
            Some(config.sender_name),
            (config.sender_mail).parse().unwrap()
        );
        let email = Message::builder()
            .from(sender_mailbox)
            .to(Mailbox::new(None, mail.to.parse()?))
            .subject(&subject)
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(mail.body)
            )?;
        let creds = Credentials::new(config.app_user, config.app_pass);
        let mserver = config.server_name;
        let mailer = SmtpTransport::starttls_relay(&mserver)?
            .credentials(creds)
            .build();
        mailer.send(&email)?;
    // --------------------------------------------------------------
        let now = Local::now();
        let s = format!("{} sent to {} {}\n", now.format("%Y-%m-%d %H:%M:%S").to_string(), mail.to, subject);
        let f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(config.log_file).unwrap();
        let mut bw = BufWriter::new(f);
        bw.write(s.as_bytes()).unwrap();
        bw.flush().unwrap();
        println!("SEND");
    } 
    Ok(())
}
