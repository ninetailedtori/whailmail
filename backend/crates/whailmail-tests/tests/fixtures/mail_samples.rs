// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

use {chrono::Utc, whailmail_common::SEmail};

pub fn sample_semail(from: &str, subject: &str) -> SEmail
{
    SEmail {
        id:              Default::default(),
        mailbox_id:      Default::default(),
        account_id:      Default::default(),
        from:            from.to_string(),
        to:              vec!["user@example.com".to_string()],
        cc:              vec![],
        bcc:             vec![],
        subject:         subject.to_string(),
        body_text:       "Test SEmail body".to_string(),
        body_html:       None,
        is_read:         false,
        is_starred:      false,
        is_archived:     false,
        received_at:     Utc::now(),
        created_at:      Default::default(),
        updated_at:      Default::default(),
        message_id:      "".to_string(),
        has_attachments: false,
        flags:           vec![]
    }
}

pub fn spam_semail() -> SEmail
{
    sample_semail("spam@example.com", "VIAGRA 50% OFF")
}

pub fn legit_semail() -> SEmail
{
    sample_semail("github@example.com", "Your deployment succeeded")
}
