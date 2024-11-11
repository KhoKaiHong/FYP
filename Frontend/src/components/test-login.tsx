import { createEffect, createSignal } from "solid-js";
import {
  userLogin,
  facilityLogin,
  organiserLogin,
  adminLogin,
} from "@/utils/login";
import {
  UserLoginResponse,
  FacilityLoginResponse,
  OrganiserLoginResponse,
  AdminLoginResponse,
} from "@/types/login";
import { useUser } from "@/context/userContext";

function LoginButtons() {
  const [icNumber, setIcNumber] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [loginResult, setLoginResult] = createSignal<string | null>(null);

  const { user, setUserStore, refreshUser } = useUser();

  const handleUserLogin = async () => {
    setUserStore({ isLoading: true });
    try {
      const response = await userLogin(icNumber(), password());
      if (response.isOk()) {
        setUserStore({
          isAuthenticated: true,
          isLoading: false,
          user: response.value.data.userDetails,
          error: null,
          role: "User",
        });
        setLoginResult(JSON.stringify(user));
      } else {
        setLoginResult(JSON.stringify(response.error));
      }
    } catch (error) {
      setLoginResult("User login failed");
    }
  };

  const handleFacilityLogin = async () => {
    setUserStore("isLoading", true);
    try {
      const response = await facilityLogin(email(), password());
      if (response.isOk()) {
        setUserStore({
          isAuthenticated: true,
          isLoading: false,
          user: response.value.data.facilityDetails,
          error: null,
          role: "Facility",
        });
        setLoginResult(JSON.stringify(user));
      } else {
        setLoginResult(JSON.stringify(response.error));
      }
    } catch (error) {
      setLoginResult("Facility login failed");
    }
  };

  const handleOrganiserLogin = async () => {
    setUserStore("isLoading", true);
    try {
      const response = await organiserLogin(email(), password());
      if (response.isOk()) {
        setUserStore({
          isAuthenticated: true,
          isLoading: false,
          user: response.value.data.organiserDetails,
          error: null,
          role: "Organiser",
        });
        setLoginResult(JSON.stringify(user));
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
        setUserStore({
          isAuthenticated: true,
          isLoading: false,
          user: response.value.data.adminDetails,
          error: null,
          role: "Admin",
        });
        setLoginResult(JSON.stringify(user));
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
      <button onClick={refreshUser}>Refetch User</button>

      {loginResult() && <p>{loginResult()}</p>}
      <p>{JSON.stringify(user)}</p>
    </div>
  );
}

export default LoginButtons;
