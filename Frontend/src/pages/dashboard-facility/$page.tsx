import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { createEffect } from "solid-js";
import { useNavigate } from "@solidjs/router";
import FaciliyProfile from "./profile-card";
import FacilityActions from "./actions";

function FacilityDashboard() {
  const { user, isLoading, refreshUser } = useUser();

  refreshUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Facility")) {
      navigate("/", { resolve: false });
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-6">
        <FaciliyProfile />
        <FacilityActions />
      </div>
    </div>
  );
}

export default FacilityDashboard;
