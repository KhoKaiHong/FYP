import Navbar from "@/components/navigation-bar";
import { BloodLevelIndicator } from "./blood-level-indicator";
import { BloodDonationSuites } from "./blood-donation-suites";
import { BloodDonationSeminars } from "./blood-donation-seminars";
import { LandingScreen } from "./landing-screen";

function Home() {
  return (
    <div>
      <Navbar />
      <LandingScreen />
      <div class="p-8 space-y-8">
        <BloodDonationSeminars />
        <BloodLevelIndicator />
        <BloodDonationSuites />
      </div>
    </div>
  );
}

export default Home;
