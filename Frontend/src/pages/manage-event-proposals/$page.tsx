import { facilityListNewEventProposal } from "@/api/new-event-proposal";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createResource, Show } from "solid-js";
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

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <Show when={eventProposals()} keyed>
          {(eventProposals) => <DataTable columns={columns} data={eventProposals} />}
        </Show>
      </div>
    </div>
  );
}

export default ManageEventProposalPage;
