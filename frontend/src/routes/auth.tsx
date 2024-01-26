import { useEffect, useRef, useState } from "react";
import { RiLockLine, RiUser3Line } from "react-icons/ri";
import { useNavigation } from "../router";
import styles from "./auth.module.scss";

let callback: () => void;

export default function Auth() {
  const [ login, setLogin ] = useState(true);
  const page =  login
    ? <LoginPage setLogin={setLogin} />
    : <RegisterPage setLogin={setLogin} />;

  return <div className={styles.background} onKeyDown={e => e.key === "Enter" && callback?.()}>
    <div className={styles.container}>
      {page}
    </div>
  </div>
}

type props = Readonly<{ setLogin: (login: boolean) => void }>;

function LoginPage({ setLogin }: props) {
  const [ disabled, setDisabled ] = useState(false);
  const { setPath } = useNavigation();

  const [ usernameError, setUsernameError ] = useState("");
  const [ passwordError, setPasswordError ] = useState("");

  const username_ref = useRef<HTMLInputElement>(null);
  const password_ref = useRef<HTMLInputElement>(null);

  useEffect(() => {
    callback = proceed;
    return () => { callback = () => {}; };
  }, []);

  function proceed() {
    if (disabled) return;

    const username = username_ref.current?.value?.trim() || "";
    const password = password_ref.current?.value?.trim() || "";

    setUsernameError("");
    setPasswordError("");

    let error = false;

    if (!username) {
      username_ref.current?.focus();
      setUsernameError("Username cannot be empty");
      error = true;
    }

    if (!password) {
      if (!error) password_ref.current?.focus();
      setPasswordError("Password cannot be empty");
      error = true;
    }

    if (error) return;
    setDisabled(true);

    (async () => {
      const res = await httpReq("/users/login", "POST", {
        body: { username, password }, withoutAuth: true,
      });

      if (res.status !== 200) {
        username_ref.current?.focus();
        setDisabled(false);
        setUsernameError("Invalid username or password");
        return;
      }

      const data = await res.text();
      localStorage.setItem("auth", data);

      setTimeout(() => setPath("/"), 750);
    })();
  }

  return <>
    <h1>Login</h1>
    <div className={styles.inputBox}>
      <p>Username</p>
      <div className={styles.input}>
        <RiUser3Line />
        <input type="text" placeholder="Username" ref={username_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} onChange={() => usernameError && setUsernameError("")} />
      </div>
      <p className={styles.error} style={{ opacity: usernameError ? 1 : 0 }}>{usernameError || "a"}</p>
    </div>
    <div className={styles.inputBox} style={{ margin: "1rem 0 2rem 0" }}>
      <p>Password</p>
      <div className={styles.input}>
        <RiLockLine />
        <input type="password" placeholder="Password" ref={password_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} onChange={() => passwordError && setPasswordError("")} />
      </div>
      <p className={styles.error} style={{ opacity: passwordError ? 1 : 0 }}>{passwordError || "a"}</p>
    </div>
    <button onClick={proceed} className={`${styles.proceed} ${disabled && styles.disabled}`}>Login</button>
    <div className={styles.loginSignup}>
      <p>Don't have an account?</p>
      <a onClick={() => disabled || setLogin(false)} className={`${disabled && styles.anchorDisabled}`} >Sign up here</a>
    </div>
  </>
}

function RegisterPage({ setLogin }: props) {
  const [ disabled, setDisabled ] = useState(false);
  const { setPath } = useNavigation();

  const [ usernameError, setUsernameError ] = useState("");
  const [ passwordError, setPasswordError ] = useState("");
  const [ confirmError, setConfirmError ] = useState("");

  const username_ref = useRef<HTMLInputElement>(null);
  const password_ref = useRef<HTMLInputElement>(null);
  const confirm_ref = useRef<HTMLInputElement>(null);

  useEffect(() => {
    callback = proceed;
    return () => { callback = () => {}; };
  }, []);

  function proceed() {
    if (disabled) return;

    const username = username_ref.current?.value?.trim() || "";
    const password = password_ref.current?.value?.trim() || "";
    const confirm = confirm_ref.current?.value?.trim() || "";

    setUsernameError("");
    setPasswordError("");
    setConfirmError("");

    let error = false;

    if (username.length < 4) {
      username_ref.current?.focus();
      setUsernameError("Username must be at least 4 characters long");
      error = true;
    }

    if (!username) {
      if (!error) username_ref.current?.focus();
      setUsernameError("Username cannot be empty");
      error = true;
    }

    if (password.length < 8) {
      if (!error) password_ref.current?.focus();
      setPasswordError("Password must be at least 8 characters long");
      error = true;
    }

    if (!password) {
      if (!error) password_ref.current?.focus();
      setPasswordError("Password cannot be empty");
      error = true;
    }

    if (password !== confirm) {
      if (!error) confirm_ref.current?.focus();
      setConfirmError("Passwords do not match");
      error = true;
    }

    if (error) return;
    setDisabled(true);

    (async () => {
      const res = await httpReq("/users/register", "POST", {
        body: { username, password }, withoutAuth: true,
      });

      if (res.status === 409) {
        username_ref.current?.focus();
        setDisabled(false);
        setUsernameError("Username already exists");
        return;
      }

      const data = await res.text();
      localStorage.setItem("auth", data);

      setTimeout(() => setPath("/"), 750);
    })();
  }

  return <>
    <h1>Register</h1>
    <div className={styles.inputBox}>
      <p>Username</p>
      <div className={styles.input}>
        <RiUser3Line />
        <input type="text" placeholder="Username" ref={username_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} onChange={() => usernameError && setUsernameError("")} />
      </div>
      <p className={styles.error} style={{ opacity: usernameError ? 1 : 0 }}>{usernameError || "a"}</p>
    </div>
    <div className={styles.inputBox} style={{ marginTop: "0.3rem" }}>
      <p>Password</p>
      <div className={styles.input}>
        <RiLockLine />
        <input type="password" placeholder="Password" ref={password_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} onChange={() => passwordError && setPasswordError("")} />
      </div>
      <p className={styles.error} style={{ opacity: passwordError ? 1 : 0 }}>{passwordError || "a"}</p>
    </div>
    <div className={styles.inputBox} style={{ margin: "0.3rem 0 1rem 0" }}>
      <p>Confirm Password</p>
      <div className={styles.input}>
        <RiLockLine />
        <input type="password" placeholder="Confirm Password" ref={confirm_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} onChange={() => confirmError && setConfirmError("")} />
      </div>
      <p className={styles.error} style={{ opacity: confirmError ? 1 : 0 }}>{confirmError || "a"}</p>
    </div>
    <button onClick={proceed} className={`${styles.proceed} ${disabled && styles.disabled}`} >Register</button>
    <div className={styles.loginSignup}>
      <p>Already have an account?</p>
      <a onClick={() => disabled || setLogin(true)} className={`${disabled && styles.anchorDisabled}`} >Login here</a>
    </div>
  </>
}