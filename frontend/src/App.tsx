import { Routes, Route, useLocationParams, useNavigation, useLocation } from "./router";

export default function App() {
  return <Routes>
    <Route path="/hello/:id/..." children={<TestRoute />} />
    <Route path="/hello2" children={<TestRoute2 />} />
    <Route path="..." children={<h1>404</h1>} />
  </Routes>
}

function TestRoute() {
  const params = useLocationParams();
  const navigation = useNavigation(); 
  const { query } = useLocation();

  return <>
    <h1>Hello {params.id} - {query.get("test")}</h1>
    <button onClick={() => navigation.setLocation(`/hello/${Math.random()}/twoja/stara`)}>Random ID</button> <br />
    <button onClick={() => navigation.setQueryParams({ test: Math.random().toString() })}>Random Query</button> <br />
    <button onClick={() => navigation.setPath("/hello2")}>Go to Hello 2</button> <br />
  </>
}

function TestRoute2() {
  const navigation = useNavigation();

  return <>
    <h1>Hello 2</h1>
    <button onClick={() => navigation.setLocation("/hello/test")}>Go to Hello</button> <br />
  </>
}