import Navbar from "@/components/navigation-bar";
import RegisterRedirectDialog from "./redirect-dialog";
import {
  Card,
  CardContent,
  CardDescription,
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
import { useNavigate, useLocation } from "@solidjs/router";
import UserRegisterForm from "./user-register-form";
import OrganiserRegisterForm from "./organiser-register-form";

function Register() {
  const navigate = useNavigate();
  const location = useLocation();

  // Get the current tab from the URL path
  const getCurrentTab = () => {
    const path = location.pathname;
    if (path === "/register/organiser") {
      return "organiser";
    } else {
      return "user";
    }
  };

  // Handle tab changes
  const handleTabChange = (value: string) => {
    if (value === "user") {
      navigate("/register/user", { resolve: false });
    } else {
      navigate("/register/organiser", { resolve: false });
    }
  };

  return (
    <div>
      <Navbar />
      <RegisterRedirectDialog />
      <div class="flex justify-center items-center min-h-[calc(100dvh-4rem)]">
        <div class="w-full max-w-4xl p-8">
          <Tabs defaultValue="user" value={getCurrentTab()} onChange={handleTabChange}>
            <TabsList>
              <TabsTrigger value="user">User</TabsTrigger>
              <TabsTrigger value="organiser">Organiser</TabsTrigger>
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
                  <OrganiserRegisterForm />
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
