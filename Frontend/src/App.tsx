import type { Component } from "solid-js";
import { Router, Route } from "@solidjs/router";
import { ColorModeProvider, ColorModeScript } from "@kobalte/core";
import { Suspense } from "solid-js";
import { UserProvider } from "@/context/userContext";

import Home from "@/routes/index";
import About from "@/routes/about";

const App: Component = () => {
  return (
    <UserProvider>
      <Router
        root={(props) => (
          <Suspense>
            <ColorModeScript />
            <ColorModeProvider>{props.children}</ColorModeProvider>
          </Suspense>
        )}
      >
        <Route path="/" component={Home} />
        <Route path="/hello-world" component={() => <h1>Hello World!</h1>} />
        <Route path="/about" component={About} />
      </Router>
    </UserProvider>
  );
};

export default App;
