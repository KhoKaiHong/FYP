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
import UserRegisterForm from "./user-register-form";
import { userLogin, organiserLogin } from "@/routes/login";
import { createSignal } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { getErrorMessage } from "@/utils/error";
import showErrorToast from "@/components/error-toast";
import { Skeleton } from "@/components/ui/skeleton";

function Register() {
  return (
    <div>
      <Navbar />
      <RegisterRedirectDialog />
      <div class="flex justify-center items-center">
        <div class="w-full max-w-4xl p-8">
          <Tabs defaultValue="user">
            <TabsList>
              <TabsTrigger value="user">User</TabsTrigger>
              <TabsTrigger value="organiser">Event Organiser</TabsTrigger>
              <TabsIndicator />
            </TabsList>
            <TabsContent value="user">
              <Card>
                <CardHeader>
                  <CardTitle>User Register</CardTitle>
                  <CardDescription>Register as a user here.</CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <UserRegisterForm />
                </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="organiser">
              <Card>
                <CardHeader>
                  <CardTitle>Event Organiser Register</CardTitle>
                  <CardDescription>
                    Register as an event organiser here.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <div class="grid grid-cols-1 space-y-8 h-full min-h-dvh py-3">
                    <Skeleton class="w-full" />
                    <Skeleton class="w-full" />
                    <Skeleton class="w-full" />
                    <Skeleton class="w-full" />
                    <Skeleton class="w-full" />
                    <Skeleton class="w-full" />
                    <Skeleton class="w-full" />
                  </div>
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </div>
  );
}

export default Register;
