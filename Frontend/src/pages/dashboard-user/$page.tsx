import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { createEffect } from "solid-js";
import { useNavigate } from "@solidjs/router";
import UserProfile from "./profile-card";
import UserActions from "./actions";

function UserDashboard() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "User")) {
      navigate("/");
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-6">
        <UserProfile />
        <UserActions />
      </div>
    </div>
  );
}

export default UserDashboard;
