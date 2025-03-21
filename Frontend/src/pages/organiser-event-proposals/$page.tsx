import { organiserListNewEventProposal } from "@/api/new-event-proposal";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createMemo, createResource, Show } from "solid-js";
import { PendingTable } from "./pending-table";
import { CompletedTable } from "./completed-table";
import { pendingColumns } from "./pendingColumns";
import { completedColumns } from "./completedColumns";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

function OrganiserEventProposalPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Organiser")) {
      navigate("/", { resolve: false });
    }
  });

  async function fetchEventProposals() {
    const result = await organiserListNewEventProposal();

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
      <div class="p-8 space-y-8">
        <Show when={eventProposals()} keyed>
          <Card class="border-2 border-brand">
            <CardHeader>
              <CardTitle>Pending New Event Proposal</CardTitle>
              <CardDescription>
                Track the status of your pending new event proposal here.
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-2">
              <PendingTable
                columns={pendingColumns}
                data={pendingEventProposals()}
              />
            </CardContent>
          </Card>
          <Card class="border-2 border-brand">
            <CardHeader>
              <CardTitle>Completed New Event Proposals</CardTitle>
              <CardDescription>
                View your past completed new event proposals.
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-2">
              <CompletedTable
                columns={completedColumns}
                data={completedEventProposals()}
              />
            </CardContent>
          </Card>
        </Show>
      </div>
    </div>
  );
}

export default OrganiserEventProposalPage;
