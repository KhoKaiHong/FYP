import { facilityListNewEventProposal } from "@/api/new-event-proposal";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createMemo, createResource, Show } from "solid-js";
import { DataTable } from "./data-table";
import { pendingColumns } from "./pendingColumns";
import { completedColumns } from "./completedColumns";
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

function ManageEventProposalPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Facility")) {
      navigate("/");
    }
  });

  async function fetchEventProposals() {
    const result = await facilityListNewEventProposal();

    return result.match(
      (data) => data.data.eventRequests,
      (error) => {
        console.error("Error fetching event proposals.", error);
        showErrorToast({
          errorTitle: "Error fetching event proposals.",
          error,
        });
        return null;
      }
    );
  }

  const [eventProposals] = createResource(fetchEventProposals);

  const pendingEventProposals = createMemo(() => {
    const eventProposalsConst = eventProposals();

    if (!eventProposalsConst) {
      return [];
    } else {
      return eventProposalsConst.filter(
        (proposal) => proposal.status === "Pending"
      );
    }
  });

  const completedEventProposals = createMemo(() => {
    const eventProposalsConst = eventProposals();

    if (!eventProposalsConst) {
      return [];
    } else {
      return eventProposalsConst.filter(
        (proposal) => proposal.status !== "Pending"
      );
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <Show when={eventProposals()} keyed>
            <Tabs defaultValue="pending">
              <TabsList>
                <TabsTrigger value="pending">Pending</TabsTrigger>
                <TabsTrigger value="completed">Completed</TabsTrigger>
                <TabsIndicator />
              </TabsList>
              <TabsContent value="pending">
                <Card class="min-h-[37rem]">
                  <CardHeader>
                    <CardTitle>Pending New Event Proposals</CardTitle>
                    <CardDescription>
                      View, approve and reject pending new event proposals.
                    </CardDescription>
                  </CardHeader>
                  <CardContent class="space-y-2">
                    <DataTable
                      columns={pendingColumns}
                      data={pendingEventProposals()}
                    />
                  </CardContent>
                </Card>
              </TabsContent>
              <TabsContent value="completed">
                <Card class="min-h-[37rem]">
                  <CardHeader>
                    <CardTitle>Completed New Event Proposals</CardTitle>
                    <CardDescription>
                      View completed new event proposals.
                    </CardDescription>
                  </CardHeader>
                  <CardContent class="space-y-2">
                    <DataTable
                      columns={completedColumns}
                      data={completedEventProposals()}
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

export default ManageEventProposalPage;
