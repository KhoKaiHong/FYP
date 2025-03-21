import { organiserListChangeEventRequest } from "@/api/change-event-request";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createMemo, createResource, Show } from "solid-js";
import { CompletedRequestsTable } from "./completed-requests-table";
import { PendingRequestsTable } from "./pending-requests-table";
import { completedRequestsColumns } from "./completedRequestsColumns";
import { pendingRequestsColumns } from "./pendingRequestsColumns";
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

function OrganiserChangeRequestsPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Organiser")) {
      navigate("/", { resolve: false });
    }
  });

  async function fetchChangeRequests() {
    const result = await organiserListChangeEventRequest();

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

  const [changeRequests] = createResource(fetchChangeRequests);

  const pendingRequests = createMemo(() => {
    const changeRequestsConst = changeRequests();

    if (!changeRequestsConst) {
      return [];
    } else {
      return changeRequestsConst.filter(
         (request) => request.status === "Pending"
      );
    }
  });

  const completedRequests = createMemo(() => {
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
                    Keep track of your progress on pending change event requests here.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <PendingRequestsTable
                    columns={pendingRequestsColumns}
                    data={pendingRequests()}
                  />
                </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="completed">
              <Card class="min-h-[37rem] border-2 border-brand">
                <CardHeader>
                  <CardTitle>Completed Change Event Requests</CardTitle>
                  <CardDescription>
                    View you completed change event requests here.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
                  <CompletedRequestsTable
                    columns={completedRequestsColumns}
                    data={completedRequests()}
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

export default OrganiserChangeRequestsPage;
