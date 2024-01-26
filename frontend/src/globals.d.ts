export {};

declare global {
  function httpReq(url: string, method: string, props?: { body?: Object, withoutAuth?: boolean }): Promise<Response>;
}

