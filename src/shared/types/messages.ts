export interface IMessage<T = never> {
  channel: string;
  data: T;
}

export interface SMessage<T = never> {
  type: string;
  payload: T;
  requestId?: string;
}
