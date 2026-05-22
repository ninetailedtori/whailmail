// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

import crypto from "crypto";

export type HashAlgorithm = "sha256" | "sha512";

export class Hash {
  private readonly _base64: string;
  private readonly _algorithm: HashAlgorithm;

  constructor(base64: string, algorithm: HashAlgorithm = "sha256") {
    if (!this.isValid(base64, algorithm)) {
      throw new Error(`Invalid ${algorithm} hash: ${base64}`);
    }
    this._base64 = base64;
    this._algorithm = algorithm;
  }

  get base64(): string {
    return this._base64;
  }

  get hex(): string {
    return Buffer.from(this._base64, "base64").toString("hex");
  }

  get bytes(): Uint8Array {
    return new Uint8Array(Buffer.from(this._base64, "base64"));
  }

  get algorithm(): HashAlgorithm {
    return this._algorithm;
  }

  static fromHex(hex: string, algorithm: HashAlgorithm = "sha256"): Hash {
    return new Hash(Buffer.from(hex, "hex").toString("base64"), algorithm);
  }

  static fromBuffer(buffer: Buffer, algorithm: HashAlgorithm = "sha256"): Hash {
    const hash = crypto.createHash(algorithm).update(buffer).digest("base64");
    return new Hash(hash, algorithm);
  }

  static fromString(data: string, algorithm: HashAlgorithm = "sha256"): Hash {
    return Hash.fromBuffer(Buffer.from(data, "utf-8"), algorithm);
  }

  toString(): string {
    return this._base64;
  }

  toJSON(): string {
    return this._base64;
  }

  equals(other: Hash): boolean {
    return this._base64 === other._base64;
  }

  private isValid(base64: string, algorithm: HashAlgorithm): boolean {
    try {
      const bytes = Buffer.from(base64, "base64");
      const expectedLength = algorithm === "sha256" ? 32 : 64;
      return bytes.length === expectedLength;
    } catch {
      return false;
    }
  }
}
