import type { Component } from "solid-js";
import { Router, Route } from "@solidjs/router";
import { ColorModeProvider, ColorModeScript } from "@kobalte/core";
import { Suspense } from "solid-js";
import { UserProvider } from "@/context/user-context";
import { ToastRegion, ToastList } from "@/components/ui/toast";

import Home from "@/pages/home/$home";
import Login from "@/pages/login/$login";
import SuperLogin from "./pages/superlogin/$superlogin";
import Register from "@/pages/register/$register";
import About from "@/pages/about";

const App: Component = () => {
  return (
    <Router
      root={(props) => (
        <>
          <ColorModeScript />
          <ColorModeProvider>
            <ToastRegion>
              <ToastList />
            </ToastRegion>
            <UserProvider>{props.children}</UserProvider>
          </ColorModeProvider>
        </>
      )}
    >
      <Route path="/" component={Home} />
      <Route path="/login" component={Login} />
      <Route path="/superlogin" component={SuperLogin} />
      <Route path="/register" component={Register} />
      <Route path="/hello-world" component={() => <h1>Hello World!</h1>} />
      <Route path="/about" component={About} />
    </Router>
  );
};

export default App;
