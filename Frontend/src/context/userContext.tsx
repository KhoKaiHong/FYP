import { createContext, useContext, ParentComponent } from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { createResource } from "solid-js";
import { fetchWithAuth } from "@/utils/fetch";
import { createEffect } from "solid-js";
import { Users } from "@/types/users";

type Role = "User" | "Facility" | "Organiser" | "Admin";

type UserState = {
  isAuthenticated: boolean;
  user: Users | null;
  error: string | null;
  isLoading: boolean;
  role: Role | null;
};

type UserContextType = UserState & {
  refreshUser: () => Promise<void>;
  setUserStore: SetStoreFunction<UserState>;
};

const UserContext = createContext<UserContextType>();

export const UserProvider: ParentComponent = (props) => {
  const [userStore, setUserStore] = createStore<UserState>({
    isAuthenticated: !!localStorage.getItem("accessToken"), // Initialize based on token existence
    user: null,
    error: null,
    isLoading: false,
    role: null,
  });

  const extractUserFromResponse = (response: {
    data: {
      userDetails?: any;
      facilityDetails?: any;
      organiserDetails?: any;
      adminDetails?: any;
    };
  }) => {
    console.log("extractUserFromResponse function called");

    if (response.data.userDetails) {
      setUserStore({ role: "User" });
      return response.data.userDetails;
    }
    if (response.data.facilityDetails) {
      setUserStore({ role: "Facility" });
      return response.data.facilityDetails;
    }
    if (response.data.organiserDetails) {
      setUserStore({ role: "Organiser" });
      return response.data.organiserDetails;
    }
    if (response.data.adminDetails) {
      setUserStore({ role: "Admin" });
      return response.data.adminDetails;
    }
    return null;
  };

  const fetchUserData = async () => {
    console.log("fetchUserData function called");

    const result = await fetchWithAuth({
      path: "/api/getcredentials",
      method: "GET",
    });

    return result.match(
      (data) => {
        const user = extractUserFromResponse(data);
        return user;
      },
      (error) => {
        if (
          error.message === "NO_AUTH" ||
          error.message === "SESSION_EXPIRED"
        ) {
          setUserStore({
            isAuthenticated: false,
            user: null,
            error: null,
            role: null,
          });
        } else {
          setUserStore({ error: error.message });
        }
        return null;
      }
    );
  };

  createEffect(() => {
    console.log("User createEffect function called");
    console.log(userStore.user);
  });

  createEffect(() => {
    console.log("isAuth createEffect function called");
    console.log(userStore.isAuthenticated);
  });

  createEffect(() => {
    console.log("isLoad createEffect function called");
    console.log(userStore.isLoading);
  });

  createEffect(() => {
    console.log("Error createEffect function called");
    console.log(userStore.error);
  });

  createEffect(() => {
    console.log("Role createEffect function called");
    console.log(userStore.role);
  });

  const [userData] = createResource(
    () => userStore.isAuthenticated,
    async () => {
      console.log("createResource function called");

      setUserStore({ isLoading: true });
      try {
        const user = await fetchUserData();
        setUserStore({ isLoading: false });
        return user;
      } catch (error) {
        setUserStore({ isLoading: false });
        console.error("Error fetching user data:", error);
        return null;
      }
    }
  );

  createEffect(() => {
    console.log("createEffect function called");

    const user = userData();
    if (user) {
      setUserStore({
        user,
        isAuthenticated: true,
        error: null,
      });
    }
  });

  const refreshUser = async () => {
    console.log("refreshUser function called");

    setUserStore({ isLoading: true });
    try {
      const user = await fetchUserData();
      setUserStore({
        isLoading: false,
        user,
        error: null,
      });
    } catch (error) {
      setUserStore({
        isLoading: false,
        error: "Failed to refresh user data",
      });
      console.error("Error refreshing user:", error);
    }
  };

  const value = {
    ...userStore,
    refreshUser,
    setUserStore,
  };

  return (
    <UserContext.Provider value={value}>{props.children}</UserContext.Provider>
  );
};

// Custom hook to use the context
export const useUser = () => {
  const context = useContext(UserContext);
  if (!context) {
    throw new Error("useUser must be used within a UserProvider");
  }
  return context;
};
