// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Data-Transfer Object

use {
    crate::{EAccountType, EFilterAction, EMailboxType, ETheme},
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize}
};

// Auth
#[derive(Debug, Serialize, Deserialize)]
pub struct SSignupReq
{
    pub email:    String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLoginReq
{
    pub email:    String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SAuthResp
{
    pub token: String,
    pub user:  SUserResp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SUserResp
{
    pub id:         String,
    pub email:      String,
    pub created_at: DateTime<Utc>
}

// Account
#[derive(Debug, Serialize, Deserialize)]
pub struct SAddAccountReq
{
    pub account_type: EAccountType,
    pub email:        String,
    pub display_name: Option<String>,
    pub imap_host:    String,
    pub imap_port:    u16,
    pub smtp_host:    String,
    pub smtp_port:    u16,
    pub username:     String,
    pub password:     String,
    pub use_tls:      bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SAccountResp
{
    pub id:           String,
    pub email:        String,
    pub display_name: Option<String>,
    pub account_type: EAccountType,
    pub unread_count: u32,
    pub last_sync:    Option<DateTime<Utc>>,
    pub created_at:   DateTime<Utc>,
    pub updated_at:   DateTime<Utc>
}

// Mailbox
#[derive(Debug, Serialize, Deserialize)]
pub struct SMailboxResp
{
    pub id:           String,
    pub account_id:   String,
    pub name:         String,
    pub imap_name:    String,
    pub mailbox_type: EMailboxType,
    pub unread_count: u32,
    pub total_count:  u32,
    pub last_sync:    Option<DateTime<Utc>>,
    pub created_at:   DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SMailboxListResp
{
    pub mailboxes: Vec<SMailboxResp>
}

// Email
#[derive(Debug, Serialize, Deserialize)]
pub struct SEmailResp<SAttachmentResp>
{
    pub id:          String,
    pub from:        String,
    pub to:          Vec<String>,
    pub cc:          Vec<String>,
    pub bcc:         Vec<String>,
    pub subject:     String,
    pub body_text:   String,
    pub body_html:   Option<String>,
    pub is_read:     bool,
    pub is_starred:  bool,
    pub is_archived: bool,
    pub received_at: DateTime<Utc>,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
    pub attachments: Vec<SAttachmentResp>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SListEmailsResp<SAttachmentResp>
{
    pub emails: Vec<SEmailResp<SAttachmentResp>>,
    pub total:  u64,
    pub limit:  u32,
    pub offset: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SSendEmailReq
{
    pub account_id:  String,
    pub to:          Vec<String>,
    pub cc:          Option<Vec<String>>,
    pub bcc:         Option<Vec<String>>,
    pub subject:     String,
    pub body_text:   String,
    pub body_html:   Option<String>,
    pub attachments: Option<Vec<String>>
}

// Attachment
#[derive(Debug, Serialize, Deserialize)]
pub struct SAttachmentResp
{
    pub id:         String,
    pub filename:   String,
    pub mime_type:  String,
    pub size_bytes: u64
}

// Filter
#[derive(Debug, Serialize, Deserialize)]
pub struct SCreateFilterReq
{
    pub name:            String,
    pub from_pattern:    Option<String>,
    pub to_pattern:      Option<String>,
    pub subject_pattern: Option<String>,
    pub body_contains:   Option<String>,
    pub actions:         Vec<EFilterAction>,
    pub enabled:         bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SFilterResp
{
    pub id:              String,
    pub user_id:         String,
    pub name:            String,
    pub from_pattern:    Option<String>,
    pub to_pattern:      Option<String>,
    pub subject_pattern: Option<String>,
    pub body_contains:   Option<String>,
    pub actions:         Vec<EFilterAction>,
    pub enabled:         bool,
    pub created_at:      DateTime<Utc>
}

// Settings
#[derive(Debug, Serialize, Deserialize)]
pub struct SUpdateSettingsReq
{
    pub theme:                 Option<ETheme>,
    pub notifications_enabled: Option<bool>,
    pub notification_sound:    Option<bool>,
    pub auto_sync_enabled:     Option<bool>,
    pub sync_interval_secs:    Option<u64>,
    pub show_avatars:          Option<bool>,
    pub reply_to_all_default:  Option<bool>
}

// Exception
#[derive(Debug, Serialize, Deserialize)]
pub struct SErrorResp
{
    pub error: String,
    pub code:  u16
}
