import type { Component } from "solid-js";
import { Router, Route } from "@solidjs/router";
import { ColorModeProvider, ColorModeScript } from "@kobalte/core";
import { UserProvider } from "@/context/user-context";
import { ToastRegion, ToastList } from "@/components/ui/toast";

import Home from "@/pages/home/$page";
import Login from "@/pages/login/$page";
import SuperLogin from "./pages/superlogin/$page";
import Register from "@/pages/register/$page";
import NotFound from "@/pages/not-found/$page";
import DonorPrivileges from "./pages/donor-privileges/$page";
import BloodDonationStatistics from "@/pages/blood-donation-statistics/$page";

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
      <Route path="*404" component={NotFound} />
    </Router>
  );
};

export default App;
