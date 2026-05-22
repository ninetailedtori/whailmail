// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

export interface FilterRule {
  // TODO: Define filter rule structure
}

export interface FilterAction {
  // TODO: Define filter action structure
}

export interface Filter {
  id: string;
  accountId: string;
  name: string;
  rules: FilterRule[];
  actions: FilterAction[];
  enabled: boolean;
}
