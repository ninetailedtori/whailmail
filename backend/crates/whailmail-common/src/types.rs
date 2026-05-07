/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EAccountType {
    Gmail,
    ProtonMail,
    Outlook,
    Custom {
        smtp_host: String,
        imap_host: String,
    },
    SelfHosted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAccount {
    pub id: String,
    pub user_id: String,
    pub account_type: EAccountType,
    pub email: String,
    pub display_name: Option<String>,
    pub imap_host: String,
    pub imap_port: u16,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
    pub last_sync: Option<DateTime<Utc>>,
    pub sync_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMailbox {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub imap_name: String,
    pub unread_count: u32,
    pub total_count: u32,
    pub last_sync: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEmail {
    pub id: String,
    pub mailbox_id: String,
    pub account_id: String,
    pub message_id: String,
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub is_archived: bool,
    pub has_attachments: bool,
    pub received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAttachment {
    pub id: String,
    pub email_id: String,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub content_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFilter {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub conditions: SFilterConditions,
    pub actions: Vec<EFilterAction>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFilterConditions {
    pub from_pattern: Option<String>,
    pub to_pattern: Option<String>,
    pub subject_pattern: Option<String>,
    pub body_contains: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EFilterAction {
    MoveTo(String),
    Delete,
    Archive,
    MarkAsRead,
    MarkAsSpam,
    Label(String),
}
