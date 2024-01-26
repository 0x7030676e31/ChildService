import { Routes, Route, useNavigation } from "./router";
import { useEffect } from "react";
import Auth from "./routes/auth";
import Home from "./routes/home";
import "./index.scss";

export default function App() {
  const { setPath } = useNavigation();
  
  useEffect(() => {
    if (!window.localStorage.getItem("auth") && window.location.pathname !== "/login") {
      console.log(Boolean(window.localStorage.getItem("auth")));
      setPath("/login");
    }
  }, []);

  return <Routes>
    <Route path="/" children={<Home />} />
    <Route path="/login" children={<Auth />} />
    <Route path="..." children={<h1>404</h1>} />
  </Routes>
}
