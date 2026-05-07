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
