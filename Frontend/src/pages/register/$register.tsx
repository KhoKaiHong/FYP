import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import RegisterRedirectDialog from "./redirect-dialog";

function Register() {
  const { user, setUser, role, setRole, isAuthenticated, setIsAuthenticated } =
    useUser();

  return (
    <div>
      <Navbar />
      <RegisterRedirectDialog />
      <div class="p-8">
        <h1 class="text-3xl font-bold mb-4">Welcome Home</h1>
        <p class="text-gray-600 mb-4">{JSON.stringify(user())}</p>
        <p class="text-gray-600 mb-4">{JSON.stringify(role())}</p>
        <p class="text-gray-600 mb-4">{JSON.stringify(isAuthenticated())}</p>
      </div>
    </div>
  );
}

export default Register;