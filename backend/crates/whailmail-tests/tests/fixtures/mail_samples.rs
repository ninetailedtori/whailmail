use chrono::Utc;
use whailmail_common::Mail;

pub fn sample_mail(from: &str, subject: &str) -> Mail {
    Mail {
        id: 1,
        user_id: 1,
        from: from.to_string(),
        to: vec!["user@example.com".to_string()],
        subject: subject.to_string(),
        body: "Test mail body".to_string(),
        received_at: Utc::now(),
    }
}

pub fn spam_mail() -> Mail {
    sample_mail("spam@example.com", "VIAGRA 50% OFF")
}

pub fn legit_mail() -> Mail {
    sample_mail("github@example.com", "Your deployment succeeded")
}
