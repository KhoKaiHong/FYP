import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { BloodLevelIndicator } from "./blood-level-indicator";

function Home() {
  const { user, isAuthenticated } = useUser();

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <BloodLevelIndicator />
      </div>
    </div>
  );
}

export default Home;
