import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { createEffect } from "solid-js";
import { useNavigate } from "@solidjs/router";
import AdminProfile from "./profile-card";
import AdminActions from "./actions";

function AdminDashboard() {
  const { user, isLoading, refreshUser } = useUser();

  refreshUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Admin")) {
      navigate("/");
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-6">
        <AdminProfile />
        <AdminActions />
      </div>
    </div>
  );
}

export default AdminDashboard;
