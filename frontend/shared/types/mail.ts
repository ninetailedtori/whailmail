// SPDX-FileCopyrightText: 2026 2026-Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

import type { EmailAddress } from "./email";

export interface MailMetadata {
  readAt?: Date;
  starredAt?: Date;
  archivedAt?: Date;
  deletedAt?: Date;
  snoozeUntil?: Date;
}

export interface Attachment {
  id: string; // hash of file
  mailId: string;
  filename: string;
  mimetype: string;
  size: number;
  localPath: string;
  cachedAt: Date;
  downloadedAt?: Date;
}

export interface Mail {
  id: string;
  accountId: string;
  from: EmailAddress;
  to: EmailAddress[];
  cc?: EmailAddress[];
  bcc?: EmailAddress[];
  subject: string;
  body: string;
  html?: string;
  headers: Record<string, string>;
  receivedAt: Date;
  metadata: MailMetadata;
  labels: string[];
  attachments: Attachment[];
}
