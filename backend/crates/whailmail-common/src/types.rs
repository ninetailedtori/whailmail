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
    crate::{
        builder_flag,
        builder_setter,
        builder_setter_opt,
        builder_vec_push,
        impl_hashed_id,
        theme::SThemeConfig
    },
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    sha1::{Digest, Sha1},
    uuid::Uuid
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSettings
{
    pub user_id:               Uuid,
    pub theme:                 SThemeConfig,
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

pub struct SEmailBuilder
{
    account_id:      Uuid,
    mailbox_id:      Uuid,
    message_id:      String,
    from:            String,
    to:              Vec<String>,
    subject:         String,
    body_text:       String,
    cc:              Vec<String>,
    bcc:             Vec<String>,
    body_html:       Option<String>,
    is_read:         bool,
    is_starred:      bool,
    is_archived:     bool,
    has_attachments: bool,
    flags:           Vec<String>
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
pub enum EFilterAction
{
    MoveTo(String),
    Delete,
    Archive,
    MarkAsRead,
    MarkAsSpam,
    Label(String)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SFilterCriterion
{
    // Pattern
    pub from_ptn: Option<String>,
    pub to_ptn:   Option<String>,
    pub cc_ptn:   Option<String>,
    pub subj_ptn: Option<String>,
    pub body_ptn: Option<String>,

    // Metadata
    pub has_attachments: Option<bool>,
    pub flags:           Option<Vec<String>>,
    pub min_size_bytes:  Option<u64>,
    pub max_size_bytes:  Option<u64>,

    // Times
    pub received_after:  Option<DateTime<Utc>>,
    pub received_before: Option<DateTime<Utc>>,

    // States
    pub is_read:    Option<bool>,
    pub is_starred: Option<bool>
}

#[derive(Debug, Default)]
pub struct SFilterCriterionBuilder
{
    from_ptn:        Option<String>,
    to_ptn:          Option<String>,
    cc_ptn:          Option<String>,
    subj_ptn:        Option<String>,
    body_ptn:        Option<String>,
    has_attachments: Option<bool>,
    flags:           Vec<String>,
    min_size_bytes:  Option<u64>,
    max_size_bytes:  Option<u64>,
    received_after:  Option<DateTime<Utc>>,
    received_before: Option<DateTime<Utc>>,
    is_read:         Option<bool>,
    is_starred:      Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFilter
{
    pub id:         Uuid,
    pub user_id:    Uuid,
    pub name:       String,
    pub criteria:   Vec<SFilterCriterion>,
    pub actions:    Vec<EFilterAction>,
    pub enabled:    bool,
    pub created_at: DateTime<Utc>
}

pub struct SFilterBuilder
{
    id:       Uuid,
    user_id:  Uuid,
    name:     String,
    criteria: Vec<SFilterCriterion>,
    actions:  Vec<EFilterAction>,
    enabled:  bool
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
pub trait TWithHashedId
{
    fn new_hashed(hash_source: String) -> Self;
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
            theme: SThemeConfig::default(),
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
        criteria: Vec<SFilterCriterion>,
        actions: Vec<EFilterAction>,
        enabled: Option<bool>
    ) -> Self
    {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            criteria,
            actions,
            enabled: enabled.unwrap_or(true),
            created_at: Utc::now()
        }
    }
}

impl_hashed_id!(SUser, email, password_hash: String);
impl_hashed_id!(SAccount, account, email, user_id: Uuid);

impl SEmailBuilder
{
    builder_setter!(from, from, String);

    builder_setter!(subject, subject, String);

    builder_setter!(body_text, body_text, String);

    builder_setter_opt!(body_html, body_html, String);

    builder_vec_push!(to, to, String);

    builder_vec_push!(cc, cc, String);

    builder_vec_push!(bcc, bcc, String);

    builder_vec_push!(flag, flags, String);

    builder_flag!(is_read, is_read);

    builder_flag!(is_starred, is_starred);

    builder_flag!(is_archived, is_archived);

    builder_flag!(has_attachments, has_attachments);

    pub fn new(account_id: Uuid, mailbox_id: Uuid, message_id: String) -> Self
    {
        Self {
            account_id,
            mailbox_id,
            message_id,
            from: String::new(),
            to: vec![],
            subject: String::new(),
            body_text: String::new(),
            cc: vec![],
            bcc: vec![],
            body_html: None,
            is_read: false,
            is_starred: false,
            is_archived: false,
            has_attachments: false,
            flags: vec![]
        }
    }

    pub fn build(self) -> SEmail
    {
        let now = Utc::now();
        SEmail {
            id:              Uuid::new_v4(),
            mailbox_id:      self.mailbox_id,
            account_id:      self.account_id,
            message_id:      self.message_id,
            from:            self.from,
            to:              self.to,
            cc:              self.cc,
            bcc:             self.bcc,
            subject:         self.subject,
            body_text:       self.body_text,
            body_html:       self.body_html,
            is_read:         self.is_read,
            is_starred:      self.is_starred,
            is_archived:     self.is_archived,
            has_attachments: self.has_attachments,
            received_at:     now,
            created_at:      now,
            updated_at:      now,
            flags:           self.flags
        }
    }
}

impl SFilterCriterionBuilder
{
    builder_setter_opt!(set_from_ptn, from_ptn, String);

    builder_setter_opt!(set_to_ptn, to_ptn, String);

    builder_setter_opt!(set_cc_ptn, cc_ptn, String);

    builder_setter_opt!(set_subj_ptn, subj_ptn, String);

    builder_setter_opt!(set_body_ptn, body_ptn, String);

    builder_setter_opt!(set_has_attachments, has_attachments, bool);

    builder_vec_push!(flag, flags, String);

    builder_setter_opt!(set_min_size, min_size_bytes, u64);

    builder_setter_opt!(set_max_size, max_size_bytes, u64);

    builder_setter_opt!(set_received_after, received_after, DateTime<Utc>);

    builder_setter_opt!(set_received_before, received_before, DateTime<Utc>);

    builder_setter_opt!(set_is_read, is_read, bool);

    builder_setter_opt!(set_is_starred, is_starred, bool);

    pub fn new() -> Self { Self::default() }

    pub fn build(self) -> SFilterCriterion
    {
        SFilterCriterion {
            from_ptn:        self.from_ptn,
            to_ptn:          self.to_ptn,
            cc_ptn:          self.cc_ptn,
            subj_ptn:        self.subj_ptn,
            body_ptn:        self.body_ptn,
            has_attachments: self.has_attachments,
            flags:           if self.flags.is_empty()
            {
                None
            }
            else
            {
                Some(self.flags)
            },
            min_size_bytes:  self.min_size_bytes,
            max_size_bytes:  self.max_size_bytes,
            received_after:  self.received_after,
            received_before: self.received_before,
            is_read:         self.is_read,
            is_starred:      self.is_starred
        }
    }
}

impl SFilterBuilder
{
    builder_vec_push!(criterion, criteria, SFilterCriterion);

    builder_vec_push!(action, actions, EFilterAction);

    builder_flag!(enabled, enabled);

    pub fn new(id: Uuid, user_id: Uuid, name: impl Into<String>) -> Self
    {
        Self {
            id,
            user_id,
            name: name.into(),
            criteria: Vec::new(),
            actions: Vec::new(),
            enabled: true
        }
    }

    pub fn build(self) -> SFilter
    {
        let now = Utc::now();
        SFilter {
            id:         self.id,
            user_id:    self.user_id,
            name:       self.name,
            criteria:   self.criteria,
            actions:    self.actions,
            enabled:    self.enabled,
            created_at: now
        }
    }
}
