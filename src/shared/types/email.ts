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
