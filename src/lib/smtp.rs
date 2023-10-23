use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use pulldown_cmark::{Parser, Options, html};
use std::fs::read_to_string;

#[derive(Debug)]
pub struct SmtpConfig {
    pub server: String,
    pub username: String,
    pub password: String
}

//static HTML_TEMPLATE: &str = "<html><body>{}</body></html>";

fn markdown_to_html_and_plain(markdown_path: &str) -> (String, String) {
    let markdown = read_to_string(markdown_path).unwrap();
    let plain = markdown_to_text::convert(&markdown); // TODO: just use the markdown text?

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let html = format!("<html><body>{}</body></html>", html_output);
    (html, plain)
}

pub fn send_mail(cfg: &SmtpConfig, markdown_path: &str) {
    let (html_text, plain_text) = markdown_to_html_and_plain(markdown_path);

    // FIXME: parameterize this!
    let email = Message::builder()
        .from("Test <test@tcfairplay.ch>".parse().unwrap())
        .to("Test <test@test.ch>".parse().unwrap())
        .subject("Test")
        .multipart(MultiPart::alternative_plain_html(plain_text, html_text))
        .unwrap();

    let creds = Credentials::new(cfg.username.clone(), cfg.password.clone());

    let mailer = SmtpTransport::relay(&cfg.server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}