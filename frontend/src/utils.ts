const http_origin = import.meta.env.DEV ? "http://localhost:2137/api" : window.location.origin;
function httpReq(url: string, method: string, props?: { body?: Object, withoutAuth?: boolean }): Promise<Response> {
  const path = http_origin + url.replace(/\/$/, "");
  const auth = props?.withoutAuth ? {} : { Authorization: localStorage.getItem("auth") ?? "" } as { Authorization: string };
  const body = props?.body ? { body: JSON.stringify(props.body) } : {};
  const contentType = props?.body ? { "Content-Type": "application/json" } : {} as { "Content-Type": string };

  return fetch(path, {
    method: method,
    headers: {
      ...auth,
      ...contentType
    },
    ...body
  });
}

globalThis.httpReq = httpReq;