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
import { facilityLogin, adminLogin } from "@/routes/login";
import { createSignal } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { getErrorMessage } from "@/utils/error";
import showErrorToast from "@/components/error-toast";

function SuperLogin() {
  const { setUser, setRole, setIsAuthenticated, setError } = useUser();
  const navigate = useNavigate();

  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");

  const [facilityEmailError, setFacilityEmailError] = createSignal("");
  const [facilityPasswordError, setFacilityPasswordError] = createSignal("");

  const [adminEmailError, setAdminEmailError] = createSignal("");
  const [adminPasswordError, setAdminPasswordError] = createSignal("");

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
      const response = await facilityLogin(email(), password());
      if (response.isOk()) {
        setIsAuthenticated(true);
        setRole("Facility");
        setUser(response.value.data.facilityDetails);
        setError(null);
        navigate("/");
      } else {
        setError(response.error);
        if (response.error.message === "EMAIL_NOT_FOUND") {
          setFacilityEmailError(getErrorMessage(response.error));
        } else if (response.error.message === "INCORRECT_PASSWORD") {
          setFacilityPasswordError(getErrorMessage(response.error));
        } else {
          showErrorToast(response.error);
        }
      }
    } catch (error) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({ message: "UNKNOWN_ERROR" });
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
      const response = await adminLogin(email(), password());
      if (response.isOk()) {
        setIsAuthenticated(true);
        setRole("Admin");
        setUser(response.value.data.adminDetails);
        setError(null);
        navigate("/");
      } else {
        setError(response.error);
        if (response.error.message === "EMAIL_NOT_FOUND") {
          setAdminEmailError(getErrorMessage(response.error));
        } else if (response.error.message === "INCORRECT_PASSWORD") {
          setAdminPasswordError(getErrorMessage(response.error));
        } else {
          showErrorToast(response.error);
        }
      }
    } catch (error) {
      setError({ message: "UNKNOWN_ERROR" });
      showErrorToast({ message: "UNKNOWN_ERROR" });
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
            onChange={() => {
              setEmail("");
              setPassword("");
              setFacilityEmailError("");
              setFacilityPasswordError("");
              setAdminEmailError("");
              setAdminPasswordError("");
            }}
          >
            <TabsList>
              <TabsTrigger value="facility">Blood Collection Facility</TabsTrigger>
              <TabsTrigger value="admin">Admin</TabsTrigger>
              <TabsIndicator />
            </TabsList>
            <TabsContent value="facility">
              <Card>
                <CardHeader>
                  <CardTitle>Blood Collection Facility Login</CardTitle>
                  <CardDescription>Login as a blood collection facility here.</CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <TextFieldRoot
                    class="space-y-1"
                    validationState={facilityEmailError() === "" ? "valid" : "invalid"}
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
                    <TextField type="password" />
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
                  <CardDescription>
                    Login as an admin here.
                  </CardDescription>
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
                    <TextField type="password" />
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
