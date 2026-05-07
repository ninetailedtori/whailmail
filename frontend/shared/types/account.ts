/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

export interface ImapSettings {
  protocol: "imap";
  host: string;
  port: number;
  secure: boolean;
  ignoreCertErrors?: boolean;
}

export interface Pop3Settings {
  protocol: "pop3";
  host: string;
  port: number;
  secure: boolean;
}

export interface ExchangeSettings {
  protocol: "exchange";
  tenantId: string;
  autodiscoverUrl?: string;
}

export type AccountSettings = ImapSettings | Pop3Settings | ExchangeSettings;

export interface Account {
  id: string;
  email: string;
  displayName: string;
  protocol: "imap" | "pop3" | "exchange";
  authType: "oauth2" | "basic" | "ntlm";
  settings: AccountSettings;
  lastSync: Date;
  syncStatus: "idle" | "syncing" | "error";
}
