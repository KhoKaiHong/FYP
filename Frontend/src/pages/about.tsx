import Navbar from "@/components/navigation-bar";
import TestFetchButton from "@/components/test-fetch-auth";
import { useUser } from "@/context/user-context";

function About() {
  const { user, role, isAuthenticated } = useUser();
  return (
    <div>
      <Navbar />
      <TestFetchButton />
      <div class="p-8">
        <p class="text-gray-600 mb-4">{JSON.stringify(user())}</p>
        <p class="text-gray-600 mb-4">{JSON.stringify(role())}</p>
        <p class="text-gray-600 mb-4">{JSON.stringify(isAuthenticated())}</p>
        <p class="text-gray-600 mb-4">
          We are a passionate team dedicated to building amazing web
          applications using Solid.js and modern web technologies.
        </p>
        <a href="/" class="text-blue-500 hover:text-blue-700 underline">
          Back to Home
        </a>
      </div>
    </div>
  );
}

export default About;
