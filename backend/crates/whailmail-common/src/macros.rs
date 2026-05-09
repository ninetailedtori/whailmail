// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Builder Macros ✨(๑•́ ω •̀๑)✨
//!
//! Tired of hand-writing the same builder setter 47 times?
//! Yeah, me too. These macros are here to be your little helpers.
//!
//! **what's inside:**
//! - `builder_setter!` — set a field, return self, no fuss
//! - `builder_setter_opt!` — same vibe but wraps it in `Some()` for you
//! - `builder_vec_push!` — append one item to a vec like a normal human
//! - `builder_flag!` — toggle bools without crying
//!
//! Just drop `builder_macro_name!(method_name, field_name, type)` in your impl
//! block and let the rust expansion fairy do the work. You're welcome :3

/// Field setter
#[macro_export]
macro_rules! builder_setter {
    ($name:ident, $field:ident, $type:ty) => {
        pub fn $name(mut self, value: impl Into<$type>) -> Self
        {
            self.$field = value.into();
            self
        }
    };
}

/// Field setter with optional wrapping
#[macro_export]
macro_rules! builder_setter_opt {
    ($name:ident, $field:ident, $type:ty) => {
        pub fn $name(mut self, value: impl Into<$type>) -> Self
        {
            self.$field = Some(value.into());
            self
        }
    };
}

/// Vector appender
#[macro_export]
macro_rules! builder_vec_push {
    ($name:ident, $field:ident, $type:ty) => {
        pub fn $name(mut self, value: impl Into<$type>) -> Self
        {
            self.$field.push(value.into());
            self
        }
    };
}

/// Boolean flag
#[macro_export]
macro_rules! builder_flag {
    ($name:ident, $field:ident) => {
        pub fn $name(mut self, value: bool) -> Self
        {
            self.$field = value;
            self
        }
    };
}

/// Generates a `.new()` constructor that auto-hashes the ID.
/// Pass the struct type, the hashing strategy (email or account+email),
/// and any extra fields. We'll fill in timestamps and hash the ID for you.
///
/// Saves you from writing the same "hash this, set timestamps, return"
/// boilerplate seventeen times.
#[macro_export]
macro_rules! impl_hashed_id {
    ($struct_type:ty,email, $($field:ident : $field_type:ty),*) => {
        impl TWithHashedId for $struct_type
        {
            fn new_hashed(email: String) -> Self
            {
                let now = Utc::now();
                Self {
                    id: hash_to_uuid(&email),
                    email,
                    password_hash: String::new(),
                    created_at: now,
                    updated_at: now
                }
            }
        }

        impl $struct_type
        {
            pub fn new(email: String, password_hash: String) -> Self
            {
                let now = Utc::now();
                Self {
                    id: hash_to_uuid(&email),
                    email,
                    password_hash,
                    created_at: now,
                    updated_at: now
                }
            }
        }
    };

    ($struct_type:ty,account,email, $($field:ident : $field_type:ty),*) => {
        impl TWithHashedId for $struct_type
        {
            fn new_hashed(email: String) -> Self
            {
                let now = Utc::now();
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
                    created_at: now,
                    updated_at: now
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
                let now = Utc::now();
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
                    created_at: now,
                    updated_at: now
                }
            }
        }
    };
}
