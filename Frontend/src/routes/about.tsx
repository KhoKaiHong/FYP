import Navbar from "@/components/navigation-bar";
import TestFetchButton from "@/components/test-fetch-auth";
import LoginButtons from "@/components/test-login";

function About() {
  return (
    <div>
      <Navbar />
      <LoginButtons />
      <TestFetchButton />
      <div class="p-8">
        <h1 class="text-3xl font-bold mb-4">About Us</h1>
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
