import { userListDonationHistory } from "@/api/donation-history";
import showErrorToast from "@/components/error-toast";
import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { createEffect, createResource, Show } from "solid-js";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { DonationHistoryTable } from "./donation-history-table";
import { donationHistoryColumns } from "./donationHistoryColumns";

function DonationHistoryPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "User")) {
      navigate("/");
    }
  });

  async function fetchDonationHistory() {
    const result = await userListDonationHistory();

    return result.match(
      (data) => data.data.donationHistory,
      (error) => {
        console.error("Error fetching donation history.", error);
        showErrorToast({
          errorTitle: "Error fetching donation history.",
          error,
        });
        return null;
      }
    );
  }

  const [donationHistory] = createResource(fetchDonationHistory);

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <Show when={donationHistory()} keyed>
          {(donationHistory) => (
            <Card class="border-2 border-brand">
              <CardHeader>
                <CardTitle>Donation History</CardTitle>
                <CardDescription>
                  View your donation history here.
                </CardDescription>
              </CardHeader>
              <CardContent class="space-y-2">
                <DonationHistoryTable
                  columns={donationHistoryColumns}
                  data={donationHistory}
                />
              </CardContent>
            </Card>
          )}
        </Show>
      </div>
    </div>
  );
}

export default DonationHistoryPage;
