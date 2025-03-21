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
import BloodDonationGuide from "@/pages/blood-donation-guide/$page";
import CommonMisconceptions from "@/pages/common-misconceptions/$page";
import DonorPrivileges from "@/pages/donor-privileges/$page";
import EventOrganisation from "@/pages/event-organisation/$page";
import BloodDonationStatistics from "@/pages/blood-donation-statistics/$page";
import Events from "@/pages/events/$page";
import UserDashboard from "@/pages/dashboard-user/$page";
import OrganiserDashboard from "@/pages/dashboard-organiser/$page";
import FacilityDashboard from "@/pages/dashboard-facility/$page";
import AdminDashboard from "@/pages/dashboard-admin/$page";
import Contact from "@/pages/contact/$page";
import EventRegistrationsPage from "@/pages/event-registrations/$page";
import DonationHistoryPage from "@/pages/donation-history/$page";
import NewEventProposalPage from "@/pages/new-event-proposal/$page";
import ManageEventProposalPage from "@/pages/manage-event-proposals/$page";
import ManageChangeRequestsPage from "@/pages/manage-change-requests/$page";
import ManageEventsPage from "./pages/manage-events/$page";
import OrganiserEventProposalPage from "@/pages/organiser-event-proposals/$page";
import OrganiserEventPage from "@/pages/organiser-events/$page";
import OrganiserChangeRequestsPage from "@/pages/organiser-change-requests/$page";
import AddAdminPage from "@/pages/add-admin/$page";
import AddFacilityPage from "@/pages/add-facility/$page";

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
      <Route path="/blood-donation-guide" component={BloodDonationGuide} />
      <Route path="/common-misconceptions" component={CommonMisconceptions} />
      <Route path="/donor-privileges" component={DonorPrivileges} />
      <Route path="event-organisation" component={EventOrganisation} />
      <Route path="/blood-donation-statistics" component={BloodDonationStatistics} />
      <Route path="/events" component={Events} />
      <Route path="/user-dashboard" component={UserDashboard} />
      <Route path="/organiser-dashboard" component={OrganiserDashboard} />
      <Route path="/facility-dashboard" component={FacilityDashboard} />
      <Route path="/admin-dashboard" component={AdminDashboard} />
      <Route path="/contact" component={Contact} />
      <Route path="/event-registrations" component={EventRegistrationsPage} />
      <Route path="/donation-history" component={DonationHistoryPage} />
      <Route path="/new-event-proposal" component={NewEventProposalPage} />
      <Route path="/manage-event-proposals" component={ManageEventProposalPage} />
      <Route path="/manage-change-requests" component={ManageChangeRequestsPage} />
      <Route path="/manage-events" component={ManageEventsPage} />
      <Route path="/organiser-event-proposals" component={OrganiserEventProposalPage} />
      <Route path="/organiser-events" component={OrganiserEventPage} />
      <Route path="/organiser-change-requests" component={OrganiserChangeRequestsPage} />
      <Route path="/add-facility" component={AddFacilityPage} />
      <Route path="/add-admin" component={AddAdminPage} />
      <Route path="*404" component={NotFound} />
    </Router>
  );
};

export default App;
