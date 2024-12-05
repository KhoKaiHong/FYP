import Navbar from "@/components/navigation-bar";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useUser } from "@/context/user-context";
import { createEffect, Show } from "solid-js";
import { Pencil } from "lucide-solid";
import { Button } from "@/components/ui/button";
import { useNavigate } from "@solidjs/router";

function UserAccount() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "User")) {
      navigate("/");
    }
  })

  function renderUserDetails() {
    const loggedInUser = user();
    if (loggedInUser && loggedInUser.role === "User") {
      return (
        <div class="grid grid-cols-2 gap-4 items-center">
          <p class="flex font-medium h-9 items-center">User ID:</p>
          <p class="flex justify-end h-9 items-center">{loggedInUser.id}</p>

          <p class="flex font-medium h-9 items-center">IC Number:</p>
          <p class="flex justify-end h-9 items-center">
            {loggedInUser.icNumber}
          </p>

          <p class="flex font-medium h-9 items-center">Name:</p>
          <p class="flex justify-end h-9 items-center">{loggedInUser.name}</p>

          <p class="flex font-medium h-9 items-center">Email:</p>
          <div class="flex items-center justify-end gap-1">
            <p>{loggedInUser.email}</p>
            <Button variant="ghost" size={"icon"}>
              <Pencil size={18} />
            </Button>
          </div>

          <p class="flex font-medium h-9 items-center">Phone Number:</p>
          <div class="flex items-center justify-end gap-1">
            <p>{loggedInUser.phoneNumber}</p>
            <Button variant="ghost" size={"icon"}>
              <Pencil size={18} />
            </Button>
          </div>

          <p class="flex font-medium h-9 items-center">Blood Type:</p>
          <p class="flex justify-end h-9 items-center">
            {loggedInUser.bloodType}
          </p>

          <p class="flex font-medium h-9 items-center">Eligibility:</p>
          <p class="flex justify-end h-9 items-center">
            {loggedInUser.eligibility}
          </p>

          <p class="flex font-medium h-9 items-center">State:</p>
          <p class="flex justify-end h-9 items-center">
            {loggedInUser.stateName}
          </p>

          <p class="flex font-medium h-9 items-center">District:</p>
          <div class="flex items-center justify-end gap-1">
            <p>{loggedInUser.districtName}</p>
            <Button variant="ghost" size={"icon"}>
              <Pencil size={18} />
            </Button>
          </div>

          <p class="flex font-medium h-9 items-center">Password:</p>
          <div class="flex items-center justify-end gap-1">
            <p>********</p>
            <Button variant="ghost" size={"icon"}>
              <Pencil size={18} />
            </Button>
          </div>
        </div>
      );
    }
  }

  return (
    <div>
      <Navbar />
      <div class="flex justify-center items-center min-h-[calc(100dvh-4rem)]">
        <div class="w-full max-w-2xl p-8">
          <Card class="border-brand border-2">
            <CardHeader>
              <CardTitle class="text-xl">User Profile</CardTitle>
              <CardDescription>
                View and edit your personal information here.
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-2">
              <Show when={user()}>{renderUserDetails()}</Show>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

export default UserAccount;
