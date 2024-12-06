import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useUser } from "@/context/user-context";
import { Show } from "solid-js";
import ChangeNameDialog from "./change-name-dialog";
import ChangeAddressDialog from "./change-address-dialog";
import ChangePasswordDialog from "./change-password-dialog";
import ChangeEmailDialog from "./change-email-dialog";
import ChangePhoneNumberDialog from "./change-phone-number-dialog";

function OrganiserProfile() {
  const { user } = useUser();

  return (
    <Card class="bg-muted shadow-none border-none">
      <CardHeader>
        <CardTitle class="text-xl">Facility Profile</CardTitle>
      </CardHeader>
      <CardContent>
        <Show when={user()} keyed>
          {(loggedInFacility) =>
            loggedInFacility.role === "Facility" ? (
              <div class="grid grid-cols-1 md:grid-cols-3 gap-y-3 md:gap-x-6 items-center justify-center">
                <div>
                  <p class="flex font-medium h-9 items-center">Facility ID:</p>
                  <p class="flex h-9 items-center">{loggedInFacility.id}</p>
                </div>

                <div>
                  <p class="flex font-medium h-9 items-center">Name:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>{loggedInFacility.name}</p>
                    <ChangeNameDialog />
                  </div>
                </div>

                <div>
                  <p class="flex font-medium h-9 items-center">Email:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>{loggedInFacility.email}</p>
                    <ChangeEmailDialog />
                  </div>
                </div>

                <div>
                  <p class="flex font-medium h-9 items-center">Phone Number:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>{loggedInFacility.phoneNumber}</p>
                    <ChangePhoneNumberDialog />
                  </div>
                </div>

                <div>
                  <p class="flex font-medium h-9 items-center">Address:</p>
                  <div class="flex items-center justify-between gap-x-2">
                    <p>{loggedInFacility.address}</p>
                    <ChangeAddressDialog />
                  </div>
                </div>

                <div>
                  <p class="flex font-medium h-9 items-center">State:</p>
                  <p class="flex h-9 items-center">{loggedInFacility.stateName}</p>
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

export default OrganiserProfile;
