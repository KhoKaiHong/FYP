import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import LoginRedirectDialog from "./redirect-dialog";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Tabs,
  TabsContent,
  TabsIndicator,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs";
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldLabel,
  TextFieldRoot,
} from "@/components/ui/text-field";
import { userLogin, organiserLogin } from "@/api/login";
import { createSignal } from "solid-js";
import { useNavigate, useLocation } from "@solidjs/router";
import { getErrorMessage } from "@/utils/error";
import showErrorToast from "@/components/error-toast";
import { Eye, EyeOff } from "lucide-solid";
import { UserLoginPayload, OrganiserLoginPayload } from "@/types/login";

function Login() {
  const navigate = useNavigate();
  const location = useLocation();

  // Get the current tab from the URL path
  const getCurrentTab = () => {
    const path = location.pathname;
    if (path === "/login/organiser") {
      return "organiser";
    } else {
      return "user";
    }
  };

  // Handle tab changes
  const handleTabChange = (value: string) => {
    setId("");
    setPassword("");
    setUserIcError("");
    setUserPasswordError("");
    setOrganiserEmailError("");
    setOrganiserPasswordError("");
    setIsPasswordVisible(false);
    if (value === "user") {
      navigate("/login/user", { resolve: false });
    } else {
      navigate("/login/organiser", { resolve: false });
    }
  };

  const { setUser, setIsAuthenticated, setError } = useUser();

  const [id, setId] = createSignal("");
  const [password, setPassword] = createSignal("");

  const [userIcError, setUserIcError] = createSignal("");
  const [userPasswordError, setUserPasswordError] = createSignal("");

  const [organiserEmailError, setOrganiserEmailError] = createSignal("");
  const [organiserPasswordError, setOrganiserPasswordError] = createSignal("");

  const [isPasswordVisible, setIsPasswordVisible] = createSignal(false);
  function togglePasswordVisibility() {
    setIsPasswordVisible(!isPasswordVisible());
  }

  function handleUserIcChange(ic: string) {
    setUserIcError("");
    setId(ic);
  }

  function handleUserPasswordChange(password: string) {
    setUserPasswordError("");
    setPassword(password);
  }

  function handleOrganiserEmailChange(email: string) {
    setOrganiserEmailError("");
    setId(email);
  }

  function handleOrganiserPasswordChange(password: string) {
    setOrganiserPasswordError("");
    setPassword(password);
  }

  async function handleUserLogin() {
    try {
      if (id() === "") {
        setUserIcError("Please enter your IC Number");
      }
      if (password() === "") {
        setUserPasswordError("Please enter your password");
      }
      if (id() === "" || password() === "") {
        return;
      }

      const response = await userLogin({
        icNumber: id(),
        password: password(),
      } as UserLoginPayload);

      if (response.isOk()) {
        setIsAuthenticated(true);
        setUser({
          ...response.value.data.userDetails,
          role: "User",
        });
        setError(null);
        navigate("/", { resolve: false });
      } else {
        setError(response.error);
        if (response.error.message === "IC_NOT_FOUND") {
          setUserIcError(getErrorMessage(response.error));
        } else if (response.error.message === "INCORRECT_PASSWORD") {
          setUserPasswordError(getErrorMessage(response.error));
        } else {
          showErrorToast({
            errorTitle: "Error during user login.",
            error: response.error,
          });
        }
      }
    } catch (error) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({
        errorTitle: "Error during user login.",
        error: { message: "UNKNOWN_ERROR" },
      });
      console.error(error);
    }
  }

  async function handleOrganiserLogin() {
    try {
      if (id() === "") {
        setOrganiserEmailError("Please enter your email");
      }
      if (password() === "") {
        setOrganiserPasswordError("Please enter your password");
      }
      if (id() === "" || password() === "") {
        return;
      }
      const response = await organiserLogin({
        email: id(),
        password: password(),
      } as OrganiserLoginPayload);
      if (response.isOk()) {
        setIsAuthenticated(true);
        setUser({
          ...response.value.data.organiserDetails,
          role: "Organiser",
        });
        setError(null);
        navigate("/", { resolve: false });
      } else {
        setError(response.error);
        if (response.error.message === "EMAIL_NOT_FOUND") {
          setOrganiserEmailError(getErrorMessage(response.error));
        } else if (response.error.message === "INCORRECT_PASSWORD") {
          setOrganiserPasswordError(getErrorMessage(response.error));
        } else {
          showErrorToast({
            errorTitle: "Error during organiser login.",
            error: response.error,
          });
        }
      }
    } catch (error) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({
        errorTitle: "Error during organiser login.",
        error: { message: "UNKNOWN_ERROR" },
      });
      console.error(error);
    }
  }

  return (
    <div>
      <Navbar />
      <LoginRedirectDialog />
      <div class="flex h-[calc(100dvh-4rem)] justify-center items-center">
        <div class="w-full max-w-4xl px-8">
          <Tabs
            defaultValue="user"
            value={getCurrentTab()}
            onChange={handleTabChange}
          >
            <TabsList>
              <TabsTrigger value="user">User</TabsTrigger>
              <TabsTrigger value="organiser">Organiser</TabsTrigger>
              <TabsIndicator />
            </TabsList>
            <TabsContent value="user">
              <Card>
                <CardHeader>
                  <CardTitle>User Login</CardTitle>
                  <CardDescription>Login as a user here.</CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={userIcError() === "" ? "valid" : "invalid"}
                    value={id()}
                    onChange={handleUserIcChange}
                  >
                    <TextFieldLabel>IC Number</TextFieldLabel>
                    <TextField placeholder="e.g. 123456-78-9012" />
                    <TextFieldErrorMessage>
                      {userIcError()}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={
                      userPasswordError() === "" ? "valid" : "invalid"
                    }
                    value={password()}
                    onChange={handleUserPasswordChange}
                  >
                    <TextFieldLabel>Password</TextFieldLabel>
                    <div class="relative">
                      <TextField
                        type={isPasswordVisible() ? "text" : "password"}
                      />
                      <button
                        class="absolute inset-y-0 end-0 flex h-full w-9 items-center justify-center rounded-e-lg text-muted-foreground/80 ring-offset-background transition-shadow hover:text-foreground focus-visible:border focus-visible:border-ring focus-visible:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                        type="button"
                        onClick={togglePasswordVisibility}
                        aria-label={
                          isPasswordVisible()
                            ? "Hide password"
                            : "Show password"
                        }
                        aria-pressed={isPasswordVisible()}
                        aria-controls="password"
                      >
                        {isPasswordVisible() ? (
                          <EyeOff
                            size={16}
                            strokeWidth={2}
                            aria-hidden="true"
                          />
                        ) : (
                          <Eye size={16} strokeWidth={2} aria-hidden="true" />
                        )}
                      </button>
                    </div>
                    <TextFieldErrorMessage>
                      {userPasswordError()}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                </CardContent>
                <CardFooter>
                  <Button onClick={handleUserLogin}>Log In</Button>
                </CardFooter>
              </Card>
            </TabsContent>
            <TabsContent value="organiser">
              <Card>
                <CardHeader>
                  <CardTitle>Event Organiser Login</CardTitle>
                  <CardDescription>
                    Login as an event organiser here.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={
                      organiserEmailError() === "" ? "valid" : "invalid"
                    }
                    value={id()}
                    onChange={handleOrganiserEmailChange}
                  >
                    <TextFieldLabel>Email Address</TextFieldLabel>
                    <TextField type="email" />
                    <TextFieldErrorMessage>
                      {organiserEmailError()}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={
                      organiserPasswordError() === "" ? "valid" : "invalid"
                    }
                    value={password()}
                    onChange={handleOrganiserPasswordChange}
                  >
                    <TextFieldLabel>Password</TextFieldLabel>
                    <div class="relative">
                      <TextField
                        type={isPasswordVisible() ? "text" : "password"}
                      />
                      <button
                        class="absolute inset-y-0 end-0 flex h-full w-9 items-center justify-center rounded-e-lg text-muted-foreground/80 ring-offset-background transition-shadow hover:text-foreground focus-visible:border focus-visible:border-ring focus-visible:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                        type="button"
                        onClick={togglePasswordVisibility}
                        aria-label={
                          isPasswordVisible()
                            ? "Hide password"
                            : "Show password"
                        }
                        aria-pressed={isPasswordVisible()}
                        aria-controls="password"
                      >
                        {isPasswordVisible() ? (
                          <EyeOff
                            size={16}
                            strokeWidth={2}
                            aria-hidden="true"
                          />
                        ) : (
                          <Eye size={16} strokeWidth={2} aria-hidden="true" />
                        )}
                      </button>
                    </div>
                    <TextFieldErrorMessage>
                      {organiserPasswordError()}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                </CardContent>
                <CardFooter>
                  <Button onClick={handleOrganiserLogin}>Log In</Button>
                </CardFooter>
              </Card>
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </div>
  );
}

export default Login;
