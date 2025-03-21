import { listRegistrationsByUserId } from "@/api/event-registration";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createMemo, createResource, Show } from "solid-js";
import { UpcomingRegistrationsTable } from "./upcoming-registrations-table";
import { upcomingRegistrationsColumns } from "./upcomingRegistrationsColumns";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { PastRegistrationsTable } from "./past-registrations-table";
import { pastRegistrationsColumns } from "./pastRegistrationsColumns";

function EventRegistrationsPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "User")) {
      navigate("/", { resolve: false });
    }
  });

  async function fetchRegistrations() {
    const result = await listRegistrationsByUserId();

    return result.match(
      (data) => data.data.registrations,
      (error) => {
        console.error("Error fetching registrations.", error);
        showErrorToast({
          errorTitle: "Error fetching registrations.",
          error,
        });
        return null;
      }
    );
  }

  const [registrations] = createResource(fetchRegistrations);

  const upcomingRegistrations = createMemo(() => {
    const registrationsConst = registrations();

    if (!registrationsConst) {
      return [];
    } else {
      return registrationsConst.filter(
        (registration) => new Date(registration.eventStartTime) > new Date()
      );
    }
  });

  const pastRegistrations = createMemo(() => {
    const registrationsConst = registrations();

    if (!registrationsConst) {
      return [];
    } else {
      return registrationsConst.filter(
        (registration) => new Date(registration.eventStartTime) <= new Date()
      );
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-8">
        <Show when={registrations()} keyed>
          <Card class="border-2 border-brand">
            <CardHeader>
              <CardTitle>Upcoming Registration</CardTitle>
              <CardDescription>
                View your registration on upcoming events here.
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-2">
              <UpcomingRegistrationsTable
                columns={upcomingRegistrationsColumns}
                data={upcomingRegistrations()}
              />
            </CardContent>
          </Card>
          <Card class="border-2 border-brand">
            <CardHeader>
              <CardTitle>Past / Ongoing Registrations</CardTitle>
              <CardDescription>
                View your registrations on past / ongoing events here.
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-2">
              <PastRegistrationsTable
                columns={pastRegistrationsColumns}
                data={pastRegistrations()}
              />
            </CardContent>
          </Card>
        </Show>
      </div>
    </div>
  );
}

export default EventRegistrationsPage;
