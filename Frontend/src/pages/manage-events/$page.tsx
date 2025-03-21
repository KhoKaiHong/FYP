import { facilityListEvents } from "@/api/events";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createMemo, createResource, Show } from "solid-js";
import { UpcomingEventsTable } from "./upcoming-events-table";
import { PastEventsTable } from "./past-events-table";
import { upcomingEventsColumns } from "./upcomingEventsColumns";
import { pastEventsColumns } from "./pastEventsColumns";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Tabs,
  TabsContent,
  TabsIndicator,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs";

function ManageEventsPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Facility")) {
      navigate("/", { resolve: false });
    }
  });

  async function fetchEvents() {
    const result = await facilityListEvents();

    return result.match(
      (data) => data.data.events,
      (error) => {
        console.error("Error fetching events.", error);
        showErrorToast({
          errorTitle: "Error fetching events.",
          error,
        });
        return null;
      }
    );
  }

  const [events] = createResource(fetchEvents);

  const upcomingEvents = createMemo(() => {
    const eventsConst = events();

    if (!eventsConst) {
      return [];
    } else {
      return eventsConst.filter(
        (event) => new Date(event.startTime) > new Date()
      );
    }
  });

  const pastEvents = createMemo(() => {
    const eventsConst = events();

    if (!eventsConst) {
      return [];
    } else {
      return eventsConst.filter(
        (event) => new Date(event.startTime) <= new Date()
      );
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <Show when={events()} keyed>
          <Tabs defaultValue="upcoming">
            <TabsList>
              <TabsTrigger value="upcoming">Upcoming</TabsTrigger>
              <TabsTrigger value="past">Past</TabsTrigger>
              <TabsIndicator />
            </TabsList>
            <TabsContent value="upcoming">
              <Card class="min-h-[37rem] border-2 border-brand">
                <CardHeader>
                  <CardTitle>Upcoming Events</CardTitle>
                  <CardDescription>
                    View details of upcoming events here.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <UpcomingEventsTable
                    columns={upcomingEventsColumns}
                    data={upcomingEvents()}
                  />
                </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="past">
              <Card class="min-h-[37rem] border-2 border-brand">
                <CardHeader>
                  <CardTitle>Past / Ongoing Events</CardTitle>
                  <CardDescription>
                    View and manage past / ongoing events here.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <PastEventsTable
                    columns={pastEventsColumns}
                    data={pastEvents()}
                  />
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
        </Show>
      </div>
    </div>
  );
}

export default ManageEventsPage;
