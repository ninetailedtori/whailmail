// SPDX-FileCopyrightText: 2026 2026-Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

export type { HashAlgorithm } from "./hash";
export { Hash } from "./hash";

export { EmailAddress } from "./email";

export type { Mail, MailMetadata, Attachment } from "./mail";

export type {
  Account,
  AccountSettings,
  ImapSettings,
  Pop3Settings,
  ExchangeSettings,
} from "./account";

export type { Filter, FilterRule, FilterAction } from "./filter";

export type { IMessage, SMessage } from "./messages";
