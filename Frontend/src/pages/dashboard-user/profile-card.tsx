import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useUser } from "@/context/user-context";
import { Show } from "solid-js";
import ChangeLocationDialog from "./change-location-dialog";
import ChangePasswordDialog from "./change-password-dialog";
import ChangeEmailDialog from "./change-email-dialog";
import ChangePhoneNumberDialog from "./change-phone-number-dialog";

function UserProfile() {
  const { user } = useUser();

  return (
    <Card class="bg-muted shadow-none border-none">
      <CardHeader>
        <CardTitle class="text-xl">User Profile</CardTitle>
      </CardHeader>
      <CardContent>
        <Show when={user()} keyed>
          {(loggedInUser) =>
            loggedInUser.role === "User" ? (
              <div class="grid grid-cols-1 md:grid-cols-3 gap-y-3 md:gap-x-6 items-center justify-center">
                <div>
                  <p class="flex font-medium h-9 items-center">User ID:</p>
                  <p class="flex h-9 items-center">{loggedInUser.id}</p>
                </div>
                <div>
                  <p class="flex font-medium h-9 items-center">IC Number:</p>
                  <p class="flex h-9 items-center">{loggedInUser.icNumber}</p>
                </div>
                <div>
                  <p class="flex font-medium h-9 items-center">Name:</p>
                  <p class="flex h-9 items-center">{loggedInUser.name}</p>
                </div>
                <div>
                  <p class="flex font-medium h-9 items-center">Email:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>{loggedInUser.email}</p>
                    <ChangeEmailDialog />
                  </div>
                </div>
                <div>
                  <p class="flex font-medium h-9 items-center">Phone Number:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>{loggedInUser.phoneNumber}</p>
                    <ChangePhoneNumberDialog />
                  </div>
                </div>
                <div>
                  <p class="flex font-medium h-9 items-center">Blood Type:</p>
                  <p class="flex h-9 items-center">{loggedInUser.bloodType}</p>
                </div>
                <div>
                  <p class="flex font-medium h-9 items-center">Eligibility:</p>
                  <p class="flex h-9 items-center">
                    {loggedInUser.eligibility}
                  </p>
                </div>
                <div>
                  <p class="flex font-medium h-9 items-center">Location:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>
                      {loggedInUser.districtName}, {loggedInUser.stateName}
                    </p>
                    <ChangeLocationDialog />
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

export default UserProfile;
