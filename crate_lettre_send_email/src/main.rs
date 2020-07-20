extern crate lettre;
extern crate lettre_email;

use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;

fn main() {
    let email_receiver = "receiver@domain.com";
    let sender = "sender@domain.com";
    let smtp_server = "smtp.exmail.qq.com";
    let password = "password"; //需要生成应用专用密码

    let email = Email::builder()
        .to(email_receiver)
        .from(sender)
        .subject("subject")
        .html("<h1>Hi there</h1>")
        .text("Message send by lettre Rust")
        .build()
        .unwrap();

    let creds = Credentials::new(sender.to_string(), password.to_string());

    // Open connection to Gmail
    let mut mailer = SmtpClient::new_simple(smtp_server)
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    print!("{:?}", result);
    mailer.close();
}
