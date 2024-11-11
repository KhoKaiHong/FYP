import { createEffect, createSignal } from "solid-js";
import {
  userLogin,
  facilityLogin,
  organiserLogin,
  adminLogin,
} from "@/routes/login";
import {
  UserLoginResponse,
  FacilityLoginResponse,
  OrganiserLoginResponse,
  AdminLoginResponse,
} from "@/types/login";
import { useUser } from "@/context/user-context";

function LoginButtons() {
  const [icNumber, setIcNumber] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [loginResult, setLoginResult] = createSignal<string | null>(null);

  const { user, setUser, role, setRole, isAuthenticated, setIsAuthenticated } =
    useUser();

  const handleUserLogin = async () => {
    try {
      const response = await userLogin(icNumber(), password());
      if (response.isOk()) {
        setUser(response.value.data.userDetails);
        setRole("User");
        setIsAuthenticated(true);

        setLoginResult(
          `${JSON.stringify(user())}, ${JSON.stringify(
            role()
          )}, ${isAuthenticated()}`
        );
      } else {
        setLoginResult(JSON.stringify(response.error));
      }
    } catch (error) {
      setLoginResult("User login failed");
    }
  };

  const handleFacilityLogin = async () => {
    try {
      const response = await facilityLogin(email(), password());
      if (response.isOk()) {
        setUser(response.value.data.facilityDetails);
        setRole("Facility");
        setIsAuthenticated(true);

        setLoginResult(
          `${JSON.stringify(user())}, ${JSON.stringify(
            role()
          )}, ${isAuthenticated()}`
        );
      } else {
        setLoginResult(JSON.stringify(response.error));
      }
    } catch (error) {
      setLoginResult("Facility login failed");
    }
  };

  const handleOrganiserLogin = async () => {
    try {
      const response = await organiserLogin(email(), password());
      if (response.isOk()) {
        setUser(response.value.data.organiserDetails);
        setRole("Organiser");
        setIsAuthenticated(true);

        setLoginResult(
          `${JSON.stringify(user())}, ${JSON.stringify(
            role()
          )}, ${isAuthenticated()}`
        );
      } else {
        setLoginResult(JSON.stringify(response.error));
      }
    } catch (error) {
      setLoginResult("Organiser login failed");
    }
  };

  const handleAdminLogin = async () => {
    try {
      const response = await adminLogin(email(), password());
      if (response.isOk()) {
        setUser(response.value.data.adminDetails);
        setRole("Admin");
        setIsAuthenticated(true);

        setLoginResult(
          `${JSON.stringify(user())}, ${JSON.stringify(
            role()
          )}, ${isAuthenticated()}`
        );
      } else {
        setLoginResult(JSON.stringify(response.error));
      }
    } catch (error) {
      setLoginResult("Admin login failed");
    }
  };

  return (
    <div>
      <input
        type="text"
        placeholder="IC Number"
        onInput={(e) => setIcNumber(e.currentTarget.value)}
        value={icNumber()}
      />
      <input
        type="email"
        placeholder="Email"
        onInput={(e) => setEmail(e.currentTarget.value)}
        value={email()}
      />
      <input
        type="password"
        placeholder="Password"
        onInput={(e) => setPassword(e.currentTarget.value)}
        value={password()}
      />

      <button onClick={handleUserLogin}>User Login</button>
      <button onClick={handleFacilityLogin}>Facility Login</button>
      <button onClick={handleOrganiserLogin}>Organiser Login</button>
      <button onClick={handleAdminLogin}>Admin Login</button>

      {loginResult() && <p>{loginResult()}</p>}
      <p>{JSON.stringify(user)}</p>
    </div>
  );
}

export default LoginButtons;
