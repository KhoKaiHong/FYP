import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";

function Home() {
  const { user, isAuthenticated } = useUser();

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <h1 class="text-3xl font-bold mb-4 text-foreground">Welcome Home</h1>
      </div>
    </div>
  );
}

export default Home;
