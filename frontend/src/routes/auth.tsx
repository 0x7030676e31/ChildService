import { useRef, useState } from "react";
import { RiLockLine, RiUser3Line } from "react-icons/ri";
import styles from "./auth.module.scss";

export default function Auth() {
  const [ login, setLogin ] = useState(true);
  const page =  login ? <LoginPage setLogin={setLogin} /> : <RegisterPage setLogin={setLogin} />;

  console.log("auth", page);

  return <div className={styles.background}>
    <div className={styles.container}>
      {page}
    </div>
  </div>
}

type props = Readonly<{ setLogin: (login: boolean) => void }>;

function LoginPage({ setLogin }: props) {
  const [ disabled, setDisabled ] = useState(false);

  const username_ref = useRef<HTMLInputElement>(null);
  const password_ref = useRef<HTMLInputElement>(null);

  function proceed() {
    if (disabled) return;

    setDisabled(true);
  }

  return <>
    <h1>Login</h1>
    <div className={styles.inputBox}>
      <p>Username</p>
      <div className={styles.input}>
        <RiUser3Line />
        <input type="text" placeholder="Username" ref={username_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} />
      </div>
    </div>
    <div className={styles.inputBox} style={{ margin: "1rem 0 2rem 0" }}>
      <p>Password</p>
      <div className={styles.input}>
        <RiLockLine />
        <input type="password" placeholder="Password" ref={password_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} />
      </div>
    </div>
    <button onClick={proceed} className={`${styles.proceed} ${disabled && styles.disabled}`} >Login</button>
    <div className={styles.loginSignup}>
      <p>Don't have an account?</p>
      <a onClick={() => disabled || setLogin(false)} className={`${disabled && styles.anchorDisabled}`} >Sign up here</a>
    </div>
  </>
}

function RegisterPage({ setLogin }: props) {
  const [ disabled, setDisabled ] = useState(false);

  const username_ref = useRef<HTMLInputElement>(null);
  const password_ref = useRef<HTMLInputElement>(null);
  const confirm_ref = useRef<HTMLInputElement>(null);

  function proceed() {
    if (disabled) return;
  }

  return <>
    <h1>Register</h1>
    <div className={styles.inputBox}>
      <p>Username</p>
      <div className={styles.input}>
        <RiUser3Line />
        <input type="text" placeholder="Username" ref={username_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} />
      </div>
    </div>
    <div className={styles.inputBox} style={{ marginTop: "1rem" }}>
      <p>Password</p>
      <div className={styles.input}>
        <RiLockLine />
        <input type="password" placeholder="Password" ref={password_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} />
      </div>
    </div>
    <div className={styles.inputBox} style={{ margin: "1rem 0 2rem 0" }}>
      <p>Confirm Password</p>
      <div className={styles.input}>
        <RiLockLine />
        <input type="password" placeholder="Confirm Password" ref={confirm_ref} className={`${disabled && styles.inputDisabled}`} disabled={disabled} />
      </div>
    </div>
    <button onClick={proceed} className={`${styles.proceed} ${disabled && styles.disabled}`} >Register</button>
    <div className={styles.loginSignup}>
      <p>Already have an account?</p>
      <a onClick={() => disabled || setLogin(true)} className={`${disabled && styles.anchorDisabled}`} >Login here</a>
    </div>
  </>
}