// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # Core Domain Types — The Soul of WhailMail ╰(*´︿`*)╯
//!
//! All our types live here: users, accounts, mailboxes, emails, filters,
//! settings.
//!
//! **Key types:**
//! - `SUser` — identity and auth
//! - `SAccount` — mailbox credentials (Gmail, ProtonMail, self-hosted, etc.)
//! - `SMailbox` — folder representation
//! - `SEmail` — message storage with flags, threading hints
//! - `SFilter` — rules for auto-organizing mail
//! - `SSettings` — user preferences (theme, notifications, sync behaviour)
//!
//! All IDs are UUID v4 strings, all timestamps are UTC, and all types derive
//! Serde. The only reason message_id is NOT UUID v4 is that this is an SMTP
//! Message-ID header (RFC 5322).

use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    sha1::{Digest, Sha1},
    uuid::Uuid
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ETheme
{
    Dark,
    Light,
    System
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSettings
{
    pub user_id:               Uuid,
    pub theme:                 ETheme,
    pub notifications_enabled: bool,
    pub notification_sound:    bool,
    pub auto_sync_enabled:     bool,
    pub sync_interval_secs:    u64,
    pub show_avatars:          bool,
    pub reply_to_all_default:  bool,
    pub created_at:            DateTime<Utc>,
    pub updated_at:            DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SUser
{
    pub id:            Uuid,
    pub email:         String,
    pub password_hash: String,
    pub created_at:    DateTime<Utc>,
    pub updated_at:    DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EAccountType
{
    Gmail,
    ProtonMail,
    Outlook,
    Custom
    {
        smtp_host: String,
        imap_host: String
    },
    SelfHosted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAccount
{
    pub id:           Uuid,
    pub user_id:      Uuid,
    pub account_type: EAccountType,
    pub email:        String,
    pub display_name: Option<String>,
    pub imap_host:    String,
    pub imap_port:    u16,
    pub smtp_host:    String,
    pub smtp_port:    u16,
    pub username:     String,
    pub password:     String,
    pub use_tls:      bool,
    pub last_sync:    Option<DateTime<Utc>>,
    pub sync_enabled: bool,
    pub created_at:   DateTime<Utc>,
    pub updated_at:   DateTime<Utc>
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EMailboxType
{
    Inbox,
    Sent,
    Drafts,
    Trash,
    Archive,
    Spam,
    Custom
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMailbox
{
    pub id:           Uuid,
    pub account_id:   Uuid,
    pub name:         String,
    pub imap_name:    String,
    pub mailbox_type: EMailboxType,
    pub unread_count: u32,
    pub total_count:  u32,
    pub last_sync:    Option<DateTime<Utc>>,
    pub created_at:   DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEmail
{
    pub id:              Uuid,
    pub mailbox_id:      Uuid,
    pub account_id:      Uuid,
    pub message_id:      String,
    pub from:            String,
    pub to:              Vec<String>,
    pub cc:              Vec<String>,
    pub bcc:             Vec<String>,
    pub subject:         String,
    pub body_text:       String,
    pub body_html:       Option<String>,
    pub is_read:         bool,
    pub is_starred:      bool,
    pub is_archived:     bool,
    pub has_attachments: bool,
    pub received_at:     DateTime<Utc>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
    pub flags:           Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAttachment
{
    pub id:          Uuid,
    pub email_id:    Uuid,
    pub filename:    String,
    pub mime_type:   String,
    pub size_bytes:  u64,
    pub content_url: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFilter
{
    pub id:         Uuid,
    pub user_id:    Uuid,
    pub name:       String,
    pub conditions: SFilterConditions,
    pub actions:    Vec<EFilterAction>,
    pub enabled:    bool,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFilterConditions
{
    pub from_pattern:    Option<String>,
    pub to_pattern:      Option<String>,
    pub subject_pattern: Option<String>,
    pub body_contains:   Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EFilterAction
{
    MoveTo(String),
    Delete,
    Archive,
    MarkAsRead,
    MarkAsSpam,
    Label(String)
}

/// Deterministically hash a string into a UUID v5.
/// Same input = same UUID forever. Great for deduping emails without
/// a separate ID table. Uses SHA-1 under the hood.
fn hash_to_uuid(data: &str) -> Uuid
{
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    let hash_result = hasher.finalize();

    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&hash_result[.. 16]);

    uuid::Builder::from_sha1_bytes(bytes).into_uuid()
}

/// Types that can be created by hashing a source string into their ID.
/// Implement this if your entity's uniqueness comes from a natural key
/// (like email) and you want that to be the primary identifier too.
pub trait WithHashedId
{
    fn new_hashed(hash_source: String) -> Self;
}

/// Generates a `.new()` constructor that auto-hashes the ID.
/// Pass the struct type, the hashing strategy (email or account+email),
/// and any extra fields. We'll fill in timestamps and hash the ID for you.
///
/// Saves you from writing the same "hash this, set timestamps, return"
/// boilerplate seventeen times.
macro_rules! impl_hashed_id {
    ($struct_type:ty,email, $($field:ident : $field_type:ty),*) => {
        impl WithHashedId for $struct_type
        {
            fn new_hashed(email: String) -> Self
            {
                Self {
                    id: hash_to_uuid(&email),
                    email,
                    password_hash: String::new(),
                    created_at: Utc::now(),
                    updated_at: Utc::now()
                }
            }
        }

        impl $struct_type
        {
            pub fn new(email: String, password_hash: String) -> Self
            {
                Self {
                    id: hash_to_uuid(&email),
                    email,
                    password_hash,
                    created_at: Utc::now(),
                    updated_at: Utc::now()
                }
            }
        }
    };

    ($struct_type:ty,account,email, $($field:ident : $field_type:ty),*) => {
        impl WithHashedId for $struct_type
        {
            fn new_hashed(email: String) -> Self
            {
                Self {
                    id: hash_to_uuid(&format!("account:{}", email)),
                    email,
                    user_id: Uuid::nil(),
                    account_type: EAccountType::Custom {
                        smtp_host: String::new(),
                        imap_host: String::new()
                    },
                    display_name: None,
                    imap_host: String::new(),
                    imap_port: 993,
                    smtp_host: String::new(),
                    smtp_port: 587,
                    username: String::new(),
                    password: String::new(),
                    use_tls: true,
                    last_sync: None,
                    sync_enabled: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now()
                }
            }
        }

        impl $struct_type
        {
            pub fn new(
                user_id: Uuid,
                email: String,
                account_type: EAccountType
            ) -> Self
            {
                Self {
                    id: hash_to_uuid(&format!("account:{}", email)),
                    user_id,
                    email,
                    account_type,
                    display_name: None,
                    imap_host: String::new(),
                    imap_port: 993,
                    smtp_host: String::new(),
                    smtp_port: 587,
                    username: String::new(),
                    password: String::new(),
                    use_tls: true,
                    last_sync: None,
                    sync_enabled: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now()
                }
            }
        }
    };
}

impl SSettings
{
    /// Create settings for a user with sensible defaults.
    /// Notifications on, dark theme, auto-sync every 5 minutes.
    pub fn new(user_id: Uuid) -> Self
    {
        let now = Utc::now();
        Self {
            user_id,
            theme: ETheme::Dark,
            notifications_enabled: true,
            notification_sound: true,
            auto_sync_enabled: true,
            sync_interval_secs: 300,
            show_avatars: true,
            reply_to_all_default: false,
            created_at: now,
            updated_at: now
        }
    }
}

impl SMailbox
{
    /// Create a mailbox for an account.
    pub fn new(
        account_id: Uuid,
        name: String,
        imap_name: String,
        mailbox_type: EMailboxType
    ) -> Self
    {
        Self {
            id: Uuid::new_v4(),
            account_id,
            name,
            imap_name,
            mailbox_type,
            unread_count: 0,
            total_count: 0,
            last_sync: None,
            created_at: Utc::now()
        }
    }
}

impl SEmail
{
    /// Create an email message in a mailbox.
    /// Starts unread, not starred, not archived. Timestamps set to now.
    pub fn new(
        mailbox_id: Uuid,
        account_id: Uuid,
        message_id: String,
        from: String,
        to: Vec<String>,
        subject: String,
        body_text: String
    ) -> Self
    {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            mailbox_id,
            account_id,
            message_id,
            from,
            to,
            cc: vec![],
            bcc: vec![],
            subject,
            body_text,
            body_html: None,
            is_read: false,
            is_starred: false,
            is_archived: false,
            has_attachments: false,
            received_at: now,
            created_at: now,
            updated_at: now,
            flags: vec![]
        }
    }
}

impl SAttachment
{
    /// Attach a file to an email.
    pub fn new(
        email_id: Uuid,
        filename: String,
        mime_type: String,
        size_bytes: u64,
        content_url: String
    ) -> Self
    {
        Self {
            id: Uuid::new_v4(),
            email_id,
            filename,
            mime_type,
            size_bytes,
            content_url
        }
    }
}

impl SFilter
{
    /// Create a mail filter rule for a user.
    /// Starts enabled. Conditions and actions are explicit.
    pub fn new(
        user_id: Uuid,
        name: String,
        conditions: SFilterConditions,
        actions: Vec<EFilterAction>
    ) -> Self
    {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            conditions,
            actions,
            enabled: true,
            created_at: Utc::now()
        }
    }
}

impl_hashed_id!(SUser, email, password_hash: String);
impl_hashed_id!(SAccount, account, email, user_id: Uuid);
