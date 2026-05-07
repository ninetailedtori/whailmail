/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Data-Transfer Object

#[derive(Debug, Serialize, Deserialize)]
pub struct SSignupReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SAuthResp {
    pub token: String,
    pub user: SUserResp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SUserResp {
    pub id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SAddAccountReq {
    pub account_type: String,
    pub email: String,
    pub display_name: Option<String>,
    pub imap_host: String,
    pub imap_port: u16,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SAccountResp {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub account_type: String,
    pub unread_count: u32,
    pub last_sync: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SEmailResp {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub received_at: DateTime<Utc>,
    pub attachments: Vec<SAttachmentResp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SAttachmentResp {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SErrorResp {
    pub error: String,
    pub code: u16,
}
