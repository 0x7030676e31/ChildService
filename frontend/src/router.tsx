import React, { useContext, useEffect, useMemo, useState } from "react";

function random_id(): number {
  return Date.now() + Math.random() * 1000;
}

const listeners: { [key: string]: () => void } = {};
const state: { query: URLSearchParams; location: string } = { query: new URLSearchParams(window.location.search), location: window.location.pathname.replace(/\/+$/, "") };

window.addEventListener("popstate", () => {
  state.query = new URLSearchParams(window.location.search);
  state.location = window.location.pathname.replace(/\/+$/, "");
  Object.values(listeners).forEach(fn => fn());
});

function refresh() {
  window.history.pushState({}, "", `${state.location}${state.query.size ? "?" : ""}${state.query.toString()}`);
  Object.values(listeners).forEach(fn => fn());
}

/**
 * Change the current location of the app (without changing the query params)
 * @param location The new location
  */
function setLocation(location: string) {
  state.location = location.replace(/\/+$/, "");
  refresh();
}

/**
 * Change the current url of the app with the given query params
 * @param path The new path
  */
function setPath(path: string) {
  const query_idx = path.indexOf("?");
  if (query_idx === -1) {
    state.query = new URLSearchParams();
    state.location = path.replace(/\/+$/, "");
  } else {
    state.query = new URLSearchParams(path.slice(query_idx + 1));
    state.location = path.slice(0, query_idx).replace(/\/+$/, "");
  }

  refresh();
}

/**
 * Change the current query params of the app
 * @param query The new query params
  */
function setQueryParams(query: { [key: string]: string }) {
  Object.entries(query).forEach(([key, value]) => {
    if (value === null) state.query.delete(key)
    else state.query.set(key, value);
  });

  refresh();
}

/**
 * Replace the current query params of the app
 * @param query The new query params
  */
function setQuery(query: { [key: string]: string }) {
  state.query = new URLSearchParams(query);
  refresh();
}

/**
 * Get the current location and query params of the app
 * @returns The current location and query params
  */
export function useLocation() {
  const id = useMemo(() => random_id(), []);
  const [_, set] = useState(0);

  useEffect(() => {
    listeners[id] = () => set(random_id());
    return () => void delete listeners[id];
  }, []);

  return state;
}

/**
 * Set of functions to change the current location and query params of the app
 * @returns Set of navigation functions
  */
export function useNavigation() {
  return { setLocation, setPath, setQueryParams, setQuery };
}

type RoutePath = Array<{ type: "string", value: string } | { type: "param", value: string } | { type: "wildcard" }>;

function parsePath(path: string): RoutePath {
  return path.split(/\/+/g).filter(Boolean).map((part) => {
    if (part.startsWith(":")) {
      return { type: "param", value: part.slice(1) };
    } else if (part === "...") {
      return { type: "wildcard" };
    } else {
      return { type: "string", value: part };
    }
  });
}

type Router = {
  [key: string]: {
    current_route: number;
    routes: Array<[RoutePath, React.ReactNode, number]>;
  }
}

const ctx = React.createContext({ provider: 0 });
const routes: Router = {};

const pathListeners: { [key: string]: { [key: string]: () => void } } = {};
const pathParams: { [key: string]: { [key: string]: string } } = {};

/**
 * Get the current params of the current route
 * @returns The current params of the current route
  */
export function useLocationParams() {
  const { provider } = useContext(ctx);
  const id = useMemo(() => random_id(), []);
  const [_, set] = useState(0);

  useEffect(() => {
    if (provider === 0) {
      throw new Error("usePath must be a child of Routes");
    }

    pathListeners[provider] ??= {};
    pathListeners[provider][id] = () => set(random_id());
    return () => void delete pathListeners[id];
  }, []);

  return pathParams[provider] ?? {};
}

export function Routes({ children }: { readonly children?: React.ReactNode }) {
  const [component, setComponent] = useState<React.ReactNode>(null);
  const id = useMemo(() => random_id(), []);

  function refresh() {
    const path = state.location.split("/").filter(Boolean);
    let params: { [key: string]: string } = {};

    const match = routes[id].routes.find(([route]) => {
      params = {};
      for (let i = 0; i < Math.max(route.length, path.length); i++) {
        const part = route[i];
        const path_part = path[i];
        if ((!path_part && part.type !== "wildcard") || (path_part && !part)) return false;

        if (part.type === "string") {
          if (part.value !== path_part) return false;
        } else if (part.type === "param") {
          params[part.value] = path_part;
        } else if (part.type === "wildcard") {
          params["..."] = path.slice(i).join("/");
          break;
        }
      }

      return true;
    });

    const are_params_equal = Object.keys(params).length === Object.keys(pathParams[id] ?? {}).length && Object.entries(params).every(([key, value]) => pathParams[id]?.[key] === value);
    if (!are_params_equal) {
      pathParams[id] = params;
      Object.values(pathListeners[id] ?? {}).forEach(fn => fn());
    }

    if (!match) {
      if (routes[id].current_route !== -1) {
        routes[id].current_route = -1;
        setComponent(null);
      }

      return;
    }
    
    const [_, component, timestamp] = match;
    if (routes[id].current_route !== timestamp) {
      routes[id].current_route = timestamp;
      setComponent(component);
    }
  }

  useEffect(() => {
    refresh();
    listeners[id] = refresh;
    return () => void delete listeners[id];
  }, [])

  return <ctx.Provider value={{ provider: id }}>
    {children}
    {component}
  </ctx.Provider>;
}

export function Route({ path, children }: { readonly path: string, readonly children?: React.ReactNode }) {
  const { provider } = useContext(ctx);
  useEffect(() => {
    if (provider === 0) {
      throw new Error("Route must be a child of Routes");
    }

    const route = parsePath(path);
    const wildcard_idx = route.findIndex(({ type }) => type === "wildcard");
    if (wildcard_idx !== -1 && wildcard_idx !== route.length - 1) {
      throw new Error("Wildcard must be the last part of a route");
    }

    routes[provider] ??= { current_route: -1, routes: [] };
    routes[provider].routes.push([route, children, random_id()]);
  }, []);

  return <></>;
}
