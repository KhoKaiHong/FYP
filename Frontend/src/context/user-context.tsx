import {
  createContext,
  useContext,
  createSignal,
  Accessor,
  Setter,
} from "solid-js";
import { createResource } from "solid-js";
import { fetchWithAuth } from "@/utils/fetch";
import { createEffect } from "solid-js";
import { Users } from "@/types/users";
import { Error } from "@/types/error";
import { GetCredentialsResponse } from "@/types/get-credentials";
import { Result } from "neverthrow";
import { logout } from "@/routes/logout";
import showErrorToast from "@/components/error-toast";

type Role = "User" | "Facility" | "Organiser" | "Admin";

type UserContextType = {
  user: Accessor<Users>;
  role: Accessor<Role>;
  isAuthenticated: Accessor<boolean>;
  error: Accessor<Error>;
  isLoading: Accessor<boolean>;
  setUser: Setter<Users>;
  setRole: Setter<Role>;
  setIsAuthenticated: Setter<boolean>;
  setError: Setter<Error>;
  refreshUser: () =>
    | Result<GetCredentialsResponse, Error>
    | Promise<Result<GetCredentialsResponse, Error>>;
  logout: () => Promise<void>;
};

const UserContext = createContext<UserContextType>();

export function UserProvider(props) {
  const [user, setUser] = createSignal<Users | null>(null);
  const [role, setRole] = createSignal<Role | null>(null);
  const [isAuthenticated, setIsAuthenticated] = createSignal(
    !!localStorage.getItem("accessToken")
  );
  const [error, setError] = createSignal<Error | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);

  async function fetchUser(): Promise<Result<GetCredentialsResponse, Error>> {
    console.log("fetchUserData function called");

    try {
      const result = await fetchWithAuth<GetCredentialsResponse>({
        path: "/api/getcredentials",
        method: "GET",
      });

      return result;
    } catch (err) {
      return err({ message: "UNKNOWN_ERROR" });
    }
  }

  const [userData, { refetch }] = createResource(
    () => isAuthenticated(),
    fetchUser
  );

  createEffect(() => {
    if (userData.error) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({ message: "UNKNOWN_ERROR" });
    }

    if (userData.loading) {
      console.log("userData.loading");
      setIsLoading(true);
    } else {
      console.log("userData.loaded");
      setIsLoading(false);
    }

    if (userData()) {
      userData().match(
        (response) => {
          if ("userDetails" in response.data) {
            setUser(response.data.userDetails);
            setRole("User");
            setIsAuthenticated(true);
            setError(null);
          } else if ("facilityDetails" in response.data) {
            setUser(response.data.facilityDetails);
            setRole("Facility");
            setIsAuthenticated(true);
            setError(null);
          } else if ("organiserDetails" in response.data) {
            setUser(response.data.organiserDetails);
            setRole("Organiser");
            setIsAuthenticated(true);
            setError(null);
          } else if ("adminDetails" in response.data) {
            setUser(response.data.adminDetails);
            setRole("Admin");
            setIsAuthenticated(true);
            setError(null);
          } else {
            setError({ message: "UNKNOWN_ERROR" });
            showErrorToast({ message: "UNKNOWN_ERROR" });
          }
        },
        (error) => {
          if (
            error.message === "NO_AUTH" ||
            error.message === "SESSION_EXPIRED"
          ) {
            setUser(null);
            setRole(null);
            setIsAuthenticated(false);
          }
          setError(error);
          showErrorToast(error);
          console.error("Error during user fetch:", error);
        }
      );
    }
  });

  async function performLogout() {
    try {
      const refreshToken = localStorage.getItem("refreshToken");
      const result = await logout(refreshToken);

      if (result.isOk()) {
        setIsAuthenticated(false);
        setRole(null);
        setUser(null);
        setError(null);
      } else {
        setError(result.error);

        if (
          result.error.message === "NO_AUTH" ||
          result.error.message === "SESSION_EXPIRED"
        ) {
          setIsAuthenticated(false);
          setRole(null);
          setUser(null);
        }
        showErrorToast(result.error);
      }
    } catch (err) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({ message: "UNKNOWN_ERROR" });
      console.error("Error during user logout:", err);
    }
  }

  const value = {
    user,
    role,
    isAuthenticated,
    error,
    isLoading,
    setUser,
    setRole,
    setIsAuthenticated,
    setError,
    refreshUser: refetch,
    logout: performLogout,
  };

  return (
    <UserContext.Provider value={value}>{props.children}</UserContext.Provider>
  );
}

export function useUser() {
  return useContext(UserContext);
}
