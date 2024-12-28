import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { BloodLevelIndicator } from "./blood-level-indicator";
import { BloodDonationSuites } from "./blood-donation-suites";

function Home() {
  const { user, isAuthenticated } = useUser();

  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-8">
        <BloodLevelIndicator />
        <BloodDonationSuites />
      </div>
    </div>
  );
}

export default Home;
