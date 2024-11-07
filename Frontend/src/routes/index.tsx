import { Component } from "solid-js";
import { Navbar } from "@/components/navigation-bar";

const Home: Component = () => {
  return (
    <div class="p-8">
      <Navbar />
      <h1 class="text-3xl font-bold mb-4">Welcome Home</h1>
      <p class="text-gray-600 mb-4">
        This is the home page of our Solid.js application. Feel free to explore!
      </p>
      <a 
        href="/about" 
        class="text-blue-500 hover:text-blue-700 underline"
      >
        Learn more about us
      </a>
    </div>
  );
};

export default Home;