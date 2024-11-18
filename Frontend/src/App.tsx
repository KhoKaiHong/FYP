import type { Component } from "solid-js";
import { Router, Route } from "@solidjs/router";
import { ColorModeProvider, ColorModeScript } from "@kobalte/core";
import { UserProvider } from "@/context/user-context";
import { ToastRegion, ToastList } from "@/components/ui/toast";

import Home from "@/pages/home/$home";
import Login from "@/pages/login/$login";
import SuperLogin from "./pages/superlogin/$superlogin";
import Register from "@/pages/register/$register";
import About from "@/pages/about";
import NotFound from "@/pages/notfound/$notfound";
import DonorPrivileges from "./pages/donor-privileges/$donor-privileges";
import BloodDonationStatistics from "@/pages/blood-donation-statistics/$blood-donation-statistics";

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
      <Route path={["/login", "/login/user", "/login/organiser"]} component={Login} />
      <Route path={["/superlogin", "/superlogin/facility", "/superlogin/admin"]} component={SuperLogin} />
      <Route path={["/register", "/register/user", "/register/organiser"]} component={Register} />
      <Route path={["/donor-privileges"]} component={DonorPrivileges} />
      <Route path={["/blood-donation-statistics"]} component={BloodDonationStatistics} />
      <Route path="/hello-world" component={() => <h1>Hello World!</h1>} />
      <Route path="/about" component={About} />
      <Route path="*404" component={NotFound} />
    </Router>
  );
};

export default App;
