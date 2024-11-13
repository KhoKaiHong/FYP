import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import RegisterRedirectDialog from "./redirect-dialog";
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
import { userLogin, organiserLogin } from "@/routes/login";
import { createSignal } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { getErrorMessage } from "@/utils/error";
import showErrorToast from "@/components/error-toast";

function Register() {
  const { setUser, setRole, setIsAuthenticated, setError } = useUser();
  const navigate = useNavigate();

  const [id, setId] = createSignal("");
  const [password, setPassword] = createSignal("");

  const [userIcError, setUserIcError] = createSignal("");
  const [userPasswordError, setUserPasswordError] = createSignal("");

  const [organiserEmailError, setOrganiserEmailError] = createSignal("");
  const [organiserPasswordError, setOrganiserPasswordError] = createSignal("");

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
      const response = await userLogin(id(), password());
      if (response.isOk()) {
        setIsAuthenticated(true);
        setRole("User");
        setUser(response.value.data.userDetails);
        setError(null);
        navigate("/");
      } else {
        setError(response.error);
        if (response.error.message === "IC_NOT_FOUND") {
          setUserIcError(getErrorMessage(response.error));
        } else if (response.error.message === "INCORRECT_PASSWORD") {
          setUserPasswordError(getErrorMessage(response.error));
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
      const response = await organiserLogin(id(), password());
      if (response.isOk()) {
        setIsAuthenticated(true);
        setRole("Organiser");
        setUser(response.value.data.organiserDetails);
        setError(null);
        navigate("/");
      } else {
        setError(response.error);
        if (response.error.message === "EMAIL_NOT_FOUND") {
          setOrganiserEmailError(getErrorMessage(response.error));
        } else if (response.error.message === "INCORRECT_PASSWORD") {
          setOrganiserPasswordError(getErrorMessage(response.error));
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
      <RegisterRedirectDialog />
      <div class="flex h-[calc(100dvh-4rem)] justify-center items-center">
        <div class="w-full max-w-4xl px-8">
          <Tabs
            defaultValue="user"
            onChange={() => {
              setId("");
              setPassword("");
              setUserIcError("");
              setUserPasswordError("");
              setOrganiserEmailError("");
              setOrganiserPasswordError("");
            }}
          >
            <TabsList>
              <TabsTrigger value="user">User</TabsTrigger>
              <TabsTrigger value="organiser">Event Organiser</TabsTrigger>
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
                    <TextField type="password" />
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
                    <TextField type="password" />
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

export default Register;