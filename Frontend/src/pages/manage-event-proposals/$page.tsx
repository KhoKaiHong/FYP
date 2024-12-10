import { facilityListNewEventProposal } from "@/api/new-event-proposal";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createMemo, createResource, Show } from "solid-js";
import { DataTable } from "./data-table";
import { columns } from "./columns";

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
          error: { message: "SERVICE_ERROR" },
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

  const nonPendingEventProposals = createMemo(() => {
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
          {(eventProposals) => (
            <div>
              <DataTable columns={columns} data={pendingEventProposals()} />
              <DataTable columns={columns} data={nonPendingEventProposals()} />
            </div>
          )}
        </Show>
      </div>
    </div>
  );
}

export default ManageEventProposalPage;
