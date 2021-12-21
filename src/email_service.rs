use lettre::{
    address::Address,
    message::Mailbox,
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        client::{Tls, TlsParameters},
        SmtpTransport,
    },
    Message, Transport,
};

use crate::{errors::AuthError, models::Confirmation, vars};

pub fn send_confirmation_mail(confirmation: &Confirmation) -> Result<(), AuthError> {
    let domain_url = vars::domain_url();
    let expires = confirmation
        .expires_at
        .format("%I:%M %p %A, %-d %B, %C%y")
        .to_string();
    let html_text = format!(
        "Please click on the link below to complete registration. <br/>
             <a href=\"{domain}/register/{id}\">Complete registration</a> <br/>
            This link expires on <strong>{expires}</strong>",
        domain = domain_url,
        id = confirmation.id,
        expires = expires
    );

    let sender = Mailbox::new(
        Some(vars::smtp_sender_name()),
        Address::new("noreply", "auth-service.com").unwrap(),
    );

    let receiver = Mailbox::new(
        Some(confirmation.email.clone()),
        Address::new("noreply", "auth-service.com").unwrap(),
    );
    let email = Message::builder()
        .to(receiver)
        .from(sender)
        .subject("Complete your registration on our one-of-a-kind Auth Service")
        .body(html_text)
        .unwrap();

    let smtp_host = vars::smtp_host();
    let tls_parameters = TlsParameters::new(smtp_host.clone()).unwrap();
    let mailer = SmtpTransport::builder_dangerous(smtp_host)
        .port(vars::smtp_port())
        .authentication(vec![Mechanism::Login])
        .credentials(Credentials::new(
            vars::smtp_username(),
            vars::smtp_password(),
        ))
        .tls(Tls::Required(tls_parameters))
        .build();
    let result = mailer.send(&email);
    if result.is_ok() {
        println!("Email sent");

        Ok(())
    } else {
        println!("Could not send email: {:?}", result);
        Err(AuthError::ProcessError(String::from(
            "Could not send confirmation email",
        )))
    }
}
