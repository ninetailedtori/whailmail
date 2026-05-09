/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

export interface IMessage<T = never> {
  channel: string;
  data: T;
}

export interface SMessage<T = never> {
  type: string;
  payload: T;
  requestId?: string;
}
