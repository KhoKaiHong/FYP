import { facilityListChangeEventRequest } from "@/api/change-event-request";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createMemo, createResource, Show } from "solid-js";
import { CompletedRequestsTable } from "./completed-requests-table";
import { PendingRequestsTable } from "./pending-requests-table";
import { pendingColumns } from "./pendingRequestsColumns";
import { completedColumns } from "./completedRequestsColumns";
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

function ManageChangeRequestsPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Facility")) {
      navigate("/");
    }
  });

  async function fetchChangeRequests() {
    const result = await facilityListChangeEventRequest();

    return result.match(
      (data) => data.data.eventRequests,
      (error) => {
        console.error("Error fetching change event requests.", error);
        showErrorToast({
          errorTitle: "Error fetching change event requests.",
          error,
        });
        return null;
      }
    );
  }

  const [changeRequests, { refetch }] = createResource(fetchChangeRequests);

  const pendingChangeRequests = createMemo(() => {
    const changeRequestsConst = changeRequests();

    if (!changeRequestsConst) {
      return [];
    } else {
      return changeRequestsConst.filter(
        (request) => request.status === "Pending"
      );
    }
  });

  const completedChangeRequests = createMemo(() => {
    const changeRequestsConst = changeRequests();

    if (!changeRequestsConst) {
      return [];
    } else {
      return changeRequestsConst.filter(
        (request) => request.status !== "Pending"
      );
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <Show when={changeRequests()} keyed>
          <Tabs defaultValue="pending">
            <TabsList>
              <TabsTrigger value="pending">Pending</TabsTrigger>
              <TabsTrigger value="completed">Completed</TabsTrigger>
              <TabsIndicator />
            </TabsList>
            <TabsContent value="pending">
              <Card class="min-h-[37rem] border-2 border-brand">
                <CardHeader>
                  <CardTitle>Pending Change Event Requests</CardTitle>
                  <CardDescription>
                    View, approve and reject pending change event requests.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <PendingRequestsTable
                    columns={pendingColumns}
                    data={pendingChangeRequests()}
                    refetch={() => {refetch()}}
                  />
                </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="completed">
              <Card class="min-h-[37rem] border-2 border-brand">
                <CardHeader>
                  <CardTitle>Completed Change Event Requests</CardTitle>
                  <CardDescription>
                    View completed change event requests.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <CompletedRequestsTable
                    columns={completedColumns}
                    data={completedChangeRequests()}
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

export default ManageChangeRequestsPage;
