use sendgrid_flows::{send_email, Email};
use slack_flows::listen_to_channel;

#[no_mangle]
pub fn run() {
    listen_to_channel("ik8", "general", |sm| {
        let email = Email {
            to: vec![String::from("jaykchen@gmail.com")],
            subject: String::from("Hi"),
            content: sm.text,
        };
        send_email("jaykchen@gmail.com", &email);
    });
}
