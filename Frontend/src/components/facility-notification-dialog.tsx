import type { DialogTriggerProps } from "@kobalte/core/dialog";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  listFacilityNotifications,
  readFacilityNotification,
} from "@/api/notifications";
import showErrorToast from "@/components/error-toast";
import { createMemo, createResource, createSignal, For, Show } from "solid-js";
import { Bell } from "lucide-solid";
import { useNavigate } from "@solidjs/router";
import {
  Pagination,
  PaginationEllipsis,
  PaginationItem,
  PaginationItems,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";

function FacilityNotificationDialog() {
  const navigate = useNavigate();

  async function fetchFacilityNotifications() {
    const result = await listFacilityNotifications();

    return result.match(
      (data) => data.data.facilityNotifications,
      (error) => {
        console.error("Error fetching facility notifications.", error);
        showErrorToast({
          errorTitle: "Error fetching facility notifications.",
          error,
        });
        return null;
      }
    );
  }

  const [facilityNotifications, { refetch }] = createResource(
    fetchFacilityNotifications
  );

  async function readNotification(notificationId: number, route?: string) {
    const result = await readFacilityNotification({ notificationId });

    result.match(
      () => {
        if (route) {
          navigate("/" + route);
        } else {
          refetch();
        }
      },
      (error) => {
        console.error("Error reading facility notifications.", error);
        showErrorToast({
          errorTitle: "Error reading facility notifications.",
          error,
        });
      }
    );
  }

  const hasUnreadNotifications = createMemo(() => {
    const notifications = facilityNotifications();
    if (!notifications) return false;
    return notifications.some((notification) => !notification.isRead);
  });

  const [currentPage, setCurrentPage] = createSignal(1);
  const pageSize = 5;

  const paginatedNotifications = createMemo(() => {
    const notifications = facilityNotifications();
    if (!notifications || notifications.length === 0) return null;

    const sorted = [...notifications].sort((a, b) => {
      if (a.isRead !== b.isRead) return a.isRead ? 1 : -1;
      return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
    });

    const start = (currentPage() - 1) * pageSize;
    return sorted.slice(start, start + pageSize);
  });

  const totalPages = createMemo(() => {
    return Math.ceil((facilityNotifications()?.length || 0) / pageSize);
  });

  return (
    <Dialog>
      <DialogTrigger
        as={(props: DialogTriggerProps) => (
          <Button variant="ghost" size={"icon"} class="relative" {...props}>
            <Bell size={18} />
            <Show when={hasUnreadNotifications()}>
              <div class="absolute top-[0.3rem] right-[0.3rem] w-2 h-2 bg-brand rounded-full" />
            </Show>
          </Button>
        )}
      />
      <DialogContent class="max-w-2xl overflow-auto">
        <DialogHeader>
          <DialogTitle>Notifications</DialogTitle>
        </DialogHeader>
        <div class="grid gap-6 py-2">
          <Show
            when={paginatedNotifications()}
            keyed
            fallback={
              <div class="text-muted-foreground">
                You have no notifications.
              </div>
            }
          >
            {(notifications) => (
              <>
                <For each={notifications}>
                  {(notification) => (
                    <div class="flex justify-between gap-x-4">
                      <div class="flex flex-col gap-y-2 w-full md:flex-row md:justify-between md:gap-x-4 md:gap-y-0 md:items-center">
                        <div class="text-sm">{notification.description}</div>
                        <div class="text-xs text-muted-foreground">
                          {new Date(notification.createdAt).toLocaleString(
                            "en-MY",
                            {
                              timeZone: "Asia/Kuala_Lumpur",
                            }
                          )}
                        </div>
                      </div>

                      <Show
                        when={notification.isRead}
                        fallback={
                          <Button
                            onClick={() =>
                              readNotification(
                                notification.id,
                                notification.redirect
                              )
                            }
                          >
                            Read
                          </Button>
                        }
                      >
                        <Button
                          variant="outline"
                          disabled={!notification.redirect}
                          onClick={() => {
                            if (notification.redirect) {
                              navigate("/" + notification.redirect);
                            }
                          }}
                        >
                          Read
                        </Button>
                      </Show>
                    </div>
                  )}
                </For>
                <div class="flex justify-center items-center">
                  <Pagination
                    count={totalPages()}
                    fixedItems
                    page={currentPage()}
                    onPageChange={setCurrentPage}
                    itemComponent={(props) => (
                      <PaginationItem page={props.page}>
                        {props.page}
                      </PaginationItem>
                    )}
                    ellipsisComponent={() => <PaginationEllipsis />}
                  >
                    <PaginationPrevious />

                    <PaginationItems />
                    <PaginationNext />
                  </Pagination>
                </div>
              </>
            )}
          </Show>
        </div>
      </DialogContent>
    </Dialog>
  );
}

export default FacilityNotificationDialog;
