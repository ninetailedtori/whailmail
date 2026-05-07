export class EmailAddress {
  readonly value: string;

  constructor(value: string) {
    if (!this.isValid(value)) {
      throw new Error(`Invalid email: ${value}`);
    }
    this.value = value;
  }

  toString(): string {
    return this.value;
  }

  equals(other: EmailAddress): boolean {
    return this.value.toLowerCase() === other.value.toLowerCase();
  }

  private isValid(email: string): boolean {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
  }
}

interface MailMetadata {
  readAt?: Date;
  starredAt?: Date;
  archivedAt?: Date;
  deletedAt?: Date;
  snoozeUntil?: Date;
}

interface Attachment {
  id: string; // hash of file
  mailId: string;
  filename: string;
  mimetype: string;
  size: number;
  localPath: string;
  cachedAt: Date;
  downloadedAt?: Date;
}

interface Mail {
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

type AccountSettings = ImapSettings | Pop3Settings | ExchangeSettings;

interface ImapSettings {
  protocol: "imap";
  host: string;
  port: number;
  secure: boolean;
  ignoreCertErrors?: boolean;
}

interface Pop3Settings {
  protocol: "pop3";
  host: string;
  port: number;
  secure: boolean;
}

interface ExchangeSettings {
  protocol: "exchange";
  tenantId: string;
  autodiscoverUrl?: string;
}

interface Account {
  id: string;
  email: string;
  displayName: string;
  protocol: "imap" | "pop3" | "exchange";
  authType: "oauth2" | "basic" | "ntlm";
  settings: AccountSettings;
  lastSync: Date;
  syncStatus: "idle" | "syncing" | "error";
}

interface Filter {
  id: string;
  accountId: string;
  name: string;
  rules: FilterRule[];
  actions: FilterAction[];
  enabled: boolean;
}

interface IMessage<T = never> {
  channel: string;
  data: T;
}

interface SMessage<T = never> {
  type: string;
  payload: T;
  requestId?: string;
}
