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
import { facilityLogin, adminLogin } from "@/api/login";
import { createSignal } from "solid-js";
import { useNavigate, useLocation } from "@solidjs/router";
import { getErrorMessage } from "@/utils/error";
import showErrorToast from "@/components/error-toast";
import { Eye, EyeOff } from "lucide-solid";
import { FacilityLoginPayload, AdminLoginPayload } from "@/types/login";

function SuperLogin() {
  const navigate = useNavigate();
  const location = useLocation();

  // Get the current tab from the URL path
  const getCurrentTab = () => {
    const path = location.pathname;
    if (path === "/superlogin/admin") {
      return "admin";
    } else {
      return "facility";
    }
  };

  // Handle tab changes
  const handleTabChange = (value: string) => {
    setEmail("");
    setPassword("");
    setFacilityEmailError("");
    setFacilityPasswordError("");
    setAdminEmailError("");
    setAdminPasswordError("");
    setIsPasswordVisible(false);
    if (value === "facility") {
      navigate("/superlogin/facility", { resolve: false });
    } else {
      navigate("/superlogin/admin", { resolve: false });
    }
  };

  const { setUser, setIsAuthenticated, setError } = useUser();

  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");

  const [facilityEmailError, setFacilityEmailError] = createSignal("");
  const [facilityPasswordError, setFacilityPasswordError] = createSignal("");

  const [adminEmailError, setAdminEmailError] = createSignal("");
  const [adminPasswordError, setAdminPasswordError] = createSignal("");

  const [isPasswordVisible, setIsPasswordVisible] = createSignal(false);
  function togglePasswordVisibility() {
    setIsPasswordVisible(!isPasswordVisible());
  }

  function handleFacilityEmailChange(email: string) {
    setFacilityEmailError("");
    setEmail(email);
  }

  function handleFacilityPasswordChange(password: string) {
    setFacilityPasswordError("");
    setPassword(password);
  }

  function handleAdminEmailChange(email: string) {
    setAdminEmailError("");
    setEmail(email);
  }

  function handleAdminPasswordChange(password: string) {
    setAdminPasswordError("");
    setPassword(password);
  }

  async function handleFacilityLogin() {
    try {
      if (email() === "") {
        setFacilityEmailError("Please enter your email");
      }
      if (password() === "") {
        setFacilityPasswordError("Please enter your password");
      }
      if (email() === "" || password() === "") {
        return;
      }

      const response = await facilityLogin({
        email: email(),
        password: password(),
      } as FacilityLoginPayload);

      if (response.isOk()) {
        setIsAuthenticated(true);
        setUser({
          ...response.value.data.facilityDetails,
          role: "Facility",
        });
        setError(null);
        navigate("/", { resolve: false });
      } else {
        setError(response.error);
        if (response.error.message === "EMAIL_NOT_FOUND") {
          setFacilityEmailError(getErrorMessage(response.error));
        } else if (response.error.message === "INCORRECT_PASSWORD") {
          setFacilityPasswordError(getErrorMessage(response.error));
        } else {
          showErrorToast({
            errorTitle: "Error during facility login.",
            error: response.error,
          });
        }
      }
    } catch (error) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({
        errorTitle: "Error during facility login.",
        error: { message: "UNKNOWN_ERROR" },
      });
      console.error(error);
    }
  }

  async function handleAdminLogin() {
    try {
      if (email() === "") {
        setAdminEmailError("Please enter your email");
      }
      if (password() === "") {
        setAdminPasswordError("Please enter your password");
      }
      if (email() === "" || password() === "") {
        return;
      }

      const response = await adminLogin({
        email: email(),
        password: password(),
      } as AdminLoginPayload);

      if (response.isOk()) {
        setIsAuthenticated(true);
        setUser({
          ...response.value.data.adminDetails,
          role: "Admin",
        });
        setError(null);
        navigate("/", { resolve: false });
      } else {
        setError(response.error);
        if (response.error.message === "EMAIL_NOT_FOUND") {
          setAdminEmailError(getErrorMessage(response.error));
        } else if (response.error.message === "INCORRECT_PASSWORD") {
          setAdminPasswordError(getErrorMessage(response.error));
        } else {
          showErrorToast({
            errorTitle: "Error during admin login.",
            error: response.error,
          });
        }
      }
    } catch (error) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({
        errorTitle: "Error during admin login.",
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
            defaultValue="facility"
            value={getCurrentTab()}
            onChange={handleTabChange}
          >
            <TabsList>
              <TabsTrigger value="facility">Facility</TabsTrigger>
              <TabsTrigger value="admin">Admin</TabsTrigger>
              <TabsIndicator />
            </TabsList>
            <TabsContent value="facility">
              <Card>
                <CardHeader>
                  <CardTitle>Blood Collection Facility Login</CardTitle>
                  <CardDescription>
                    Login as a blood collection facility here.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={
                      facilityEmailError() === "" ? "valid" : "invalid"
                    }
                    value={email()}
                    onChange={handleFacilityEmailChange}
                  >
                    <TextFieldLabel>Email Address</TextFieldLabel>
                    <TextField type="email" />
                    <TextFieldErrorMessage>
                      {facilityEmailError()}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={
                      facilityPasswordError() === "" ? "valid" : "invalid"
                    }
                    value={password()}
                    onChange={handleFacilityPasswordChange}
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
                      {facilityPasswordError()}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                </CardContent>
                <CardFooter>
                  <Button onClick={handleFacilityLogin}>Log In</Button>
                </CardFooter>
              </Card>
            </TabsContent>
            <TabsContent value="admin">
              <Card>
                <CardHeader>
                  <CardTitle>Admin Login</CardTitle>
                  <CardDescription>Login as an admin here.</CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={
                      adminEmailError() === "" ? "valid" : "invalid"
                    }
                    value={email()}
                    onChange={handleAdminEmailChange}
                  >
                    <TextFieldLabel>Email Address</TextFieldLabel>
                    <TextField type="email" />
                    <TextFieldErrorMessage>
                      {adminEmailError()}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={
                      adminPasswordError() === "" ? "valid" : "invalid"
                    }
                    value={password()}
                    onChange={handleAdminPasswordChange}
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
                      {adminPasswordError()}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                </CardContent>
                <CardFooter>
                  <Button onClick={handleAdminLogin}>Log In</Button>
                </CardFooter>
              </Card>
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </div>
  );
}

export default SuperLogin;
