import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useUser } from "@/context/user-context";
import { Show } from "solid-js";
import ChangeNameDialog from "./change-name-dialog";
import ChangePasswordDialog from "./change-password-dialog";
import ChangeEmailDialog from "./change-email-dialog";

function AdminProfile() {
  const { user } = useUser();

  return (
    <Card class="bg-muted shadow-none border-none">
      <CardHeader>
        <CardTitle class="text-xl">Admin Profile</CardTitle>
      </CardHeader>
      <CardContent>
        <Show when={user()} keyed>
          {(loggedInAdmin) =>
            loggedInAdmin.role === "Admin" ? (
              <div class="grid grid-cols-1 md:grid-cols-2 gap-y-3 md:gap-x-6 items-center justify-center">
                <div>
                  <p class="flex font-medium h-9 items-center">Admin ID:</p>
                  <p class="flex h-9 items-center">{loggedInAdmin.id}</p>
                </div>

                <div>
                  <p class="flex font-medium h-9 items-center">Name:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>{loggedInAdmin.name}</p>
                    <ChangeNameDialog />
                  </div>
                </div>

                <div>
                  <p class="flex font-medium h-9 items-center">Email:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>{loggedInAdmin.email}</p>
                    <ChangeEmailDialog />
                  </div>
                </div>

                <div>
                  <p class="flex font-medium h-9 items-center">Password:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>********</p>
                    <ChangePasswordDialog />
                  </div>
                </div>
              </div>
            ) : null
          }
        </Show>
      </CardContent>
    </Card>
  );
}

export default AdminProfile;
