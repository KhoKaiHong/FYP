import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";

function Home() {
  const { user, role, isAuthenticated } = useUser();

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <h1 class="text-3xl font-bold mb-4">Welcome Home</h1>
        <p class="text-gray-600 mb-4">{JSON.stringify(user())}</p>
        <p class="text-gray-600 mb-4">{JSON.stringify(role())}</p>
        <p class="text-gray-600 mb-4">{JSON.stringify(isAuthenticated())}</p>
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
        <a href="/about" class="text-blue-500 hover:text-blue-700 underline">
          Learn more about us
        </a>
      </div>
    </div>
  );
}

export default Home;
