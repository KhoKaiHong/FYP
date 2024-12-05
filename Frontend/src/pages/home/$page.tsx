import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";

function Home() {
  const { user, isAuthenticated } = useUser();

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <h1 class="text-3xl font-bold mb-4 text-foreground">Welcome Home</h1>
        <p class="text-foreground mb-4">{JSON.stringify(user())}</p>
        <p class="text-foreground mb-4">{JSON.stringify(isAuthenticated())}</p>
        <p class="text-foreground mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-500 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-700 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-slate-400 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-slate-500 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-slate-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-slate-700 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
        <p class="text-gray-600 mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
      </div>
      <div class="w-full bg-muted h-48">
        <p class="text-muted-foreground mb-4">
          This is the home page of our Solid.js application. Feel free to
          explore!
        </p>
      </div>
    </div>
  );
}

export default Home;
