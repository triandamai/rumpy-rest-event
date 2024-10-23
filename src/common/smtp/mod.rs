use lettre::{Message, SmtpTransport, Transport};

const URL_SMTP_VAR: &str = "SMPTP";
const URL_SMTP_DEV: &str = "smpts://username:password@sirkel.com:465";

pub struct SmtpClient {
    from: String,
    reply_to: Option<String>,
    to: String,
}


impl SmtpClient {
    pub fn new(to: &String) -> SmtpClient {
        SmtpClient {
            from: String::from("Sirkel <sirkel@gmail.com>"),
            to: to.clone(),
            reply_to: None,
        }
    }
    pub fn new_with_reply(to: &String, reply_to: &String) -> SmtpClient {
        return SmtpClient {
            from: String::from("Sirkel <sirkel@gmail.com>"),
            to: to.clone(),
            reply_to: Some(reply_to.clone()),
        };
    }

    //smtps:// username:password@smtp. example. com:465
    pub async fn send(&self, subject: &String, body: &String) {
        let message = match self.reply_to.clone() {
            None => Message::builder()
                .from(self.from.parse().unwrap())
                .reply_to("No Reply <noreply@sirkel.com>".parse().unwrap())
                .to(self.to.parse().unwrap())
                .subject(subject.clone())
                .body(body.clone())
                .unwrap(),

            Some(reply) => Message::builder()
                .from(self.from.parse().unwrap())
                .reply_to(reply.parse().unwrap())
                .to(self.to.parse().unwrap())
                .subject(subject.clone())
                .body(body.clone())
                .unwrap()
        };
        let url = std::env::var(URL_SMTP_VAR)
            .unwrap_or(URL_SMTP_DEV.to_string());

        let mailer = SmtpTransport::from_url(url.as_str());
        if !mailer.is_err() {
            let mailer = mailer.unwrap().build();

            let _ = mailer.send(&message);
        }

    }
}