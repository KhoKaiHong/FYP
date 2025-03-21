import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { createEffect } from "solid-js";
import { useNavigate } from "@solidjs/router";
import OrganiserProfile from "./profile-card";
import OrganiserActions from "./actions";

function OrganiserDashboard() {
  const { user, isLoading, refreshUser } = useUser();

  refreshUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Organiser")) {
      navigate("/", { resolve: false });
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-6">
        <OrganiserProfile />
        <OrganiserActions />
      </div>
    </div>
  );
}

export default OrganiserDashboard;
