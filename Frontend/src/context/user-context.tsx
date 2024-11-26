import {
  createContext,
  useContext,
  createSignal,
  Accessor,
  Setter,
  JSXElement,
} from "solid-js";
import { createResource } from "solid-js";
import { fetchWithAuth } from "@/utils/fetch-auth";
import { createEffect } from "solid-js";
import { Users } from "@/types/users";
import { AppError } from "@/types/error";
import { GetCredentialsResponse } from "@/types/get-credentials";
import { Result, err } from "neverthrow";
import { logout } from "@/api/logout";
import showErrorToast from "@/components/error-toast";
import { LogoutPayload } from "@/types/logout";

type Role = "User" | "Facility" | "Organiser" | "Admin";

type UserContextType = {
  user: Accessor<Users | null>;
  role: Accessor<Role | null>;
  isAuthenticated: Accessor<boolean>;
  error: Accessor<AppError | null>;
  isLoading: Accessor<boolean>;
  setUser: Setter<Users | null>;
  setRole: Setter<Role | null>;
  setIsAuthenticated: Setter<boolean>;
  setError: Setter<AppError | null>;
  refreshUser: () =>
    | Result<GetCredentialsResponse, AppError>
    | Promise<Result<GetCredentialsResponse, AppError> | undefined>
    | undefined
    | null;
  logout: () => Promise<void>;
};

const UserContext = createContext<UserContextType>();

type UserProviderProps = {
  children?: JSXElement;
};

export function UserProvider(props: UserProviderProps) {
  const [user, setUser] = createSignal<Users | null>(null);
  const [role, setRole] = createSignal<Role | null>(null);
  const [isAuthenticated, setIsAuthenticated] = createSignal(
    !!localStorage.getItem("accessToken")
  );
  const [error, setError] = createSignal<AppError | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);

  async function fetchUser(): Promise<
    Result<GetCredentialsResponse, AppError>
  > {
    console.log("fetchUserData function called");

    try {
      const result = await fetchWithAuth<GetCredentialsResponse>({
        path: "/api/get-credentials",
        method: "GET",
      });

      return result;
    } catch (error) {
      console.error("Error during user fetch:", error);
      return err({ message: "UNKNOWN_ERROR" });
    }
  }

  const [userData, { refetch }] = createResource(
    () => isAuthenticated(),
    fetchUser
  );

  createEffect(() => {
    if (userData.loading) {
      console.log("userData.loading");
      setIsLoading(true);
      return;
    }

    setIsLoading(false);
    console.log("userData.loaded");

    if (userData.error) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({
        errorTitle: "Error loading user data.",
        error: { message: "UNKNOWN_ERROR" },
      });
      return;
    }

    const data = userData();
    if (!data) {
      setIsAuthenticated(false);
      setRole(null);
      setUser(null);
      console.log("data is null");
      return;
    }

    data.match(
      (response) => {
        if ("userDetails" in response.data) {
          setIsAuthenticated(true);
          setRole("User");
          setUser(response.data.userDetails);
          setError(null);
        } else if ("facilityDetails" in response.data) {
          setIsAuthenticated(true);
          setRole("Facility");
          setUser(response.data.facilityDetails);
          setError(null);
        } else if ("organiserDetails" in response.data) {
          setIsAuthenticated(true);
          setRole("Organiser");
          setUser(response.data.organiserDetails);
          setError(null);
        } else if ("adminDetails" in response.data) {
          setIsAuthenticated(true);
          setRole("Admin");
          setUser(response.data.adminDetails);
          setError(null);
        } else {
          setError({ message: "UNKNOWN_ERROR" });
          showErrorToast({
            errorTitle: "Error loading user data.",
            error: { message: "UNKNOWN_ERROR" },
          });
        }
      },
      (error) => {
        if (
          error.message === "NO_AUTH" ||
          error.message === "SESSION_EXPIRED"
        ) {
          setIsAuthenticated(false);
          setRole(null);
          setUser(null);
        }
        setError(error);
        showErrorToast({ errorTitle: "Error loading user data.", error });
        console.error("Error during user fetch:", error);
      }
    );
  });

  async function performLogout() {
    try {
      const refreshToken = localStorage.getItem("refreshToken");
      if (!refreshToken) {
        localStorage.removeItem("accessToken");
        setIsAuthenticated(false);
        setRole(null);
        setUser(null);
        setError({ message: "NO_AUTH" });
        showErrorToast({
          errorTitle: "Error during log out.",
          error: { message: "NO_AUTH" },
        });
        return;
      }

      const result = await logout({
        refreshToken: refreshToken,
      } as LogoutPayload);

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
        showErrorToast({
          errorTitle: "Error during log out.",
          error: result.error,
        });
      }
    } catch (err) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({
        errorTitle: "Error during log out.",
        error: { message: "UNKNOWN_ERROR" },
      });
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
  const context = useContext(UserContext);
  if (!context) {
    showErrorToast({
      errorTitle: "Error loading user data.",
      error: { message: "UNKNOWN_ERROR" },
    });
    console.error("Cannot find UserContext");
    throw new Error("Cannot find UserContext");
  }
  return context;
}
