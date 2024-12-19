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
  listUserNotifications,
  readUserNotification,
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

function UserNotificationDialog() {
  const navigate = useNavigate();

  async function fetchUserNotifications() {
    const result = await listUserNotifications();

    return result.match(
      (data) => data.data.userNotifications,
      (error) => {
        console.error("Error fetching user notifications.", error);
        showErrorToast({
          errorTitle: "Error fetching user notifications.",
          error,
        });
        return null;
      }
    );
  }

  const [userNotifications, { refetch }] = createResource(
    fetchUserNotifications
  );

  async function readNotification(notificationId: number, route?: string) {
    const result = await readUserNotification({ notificationId });

    result.match(
      () => {
        if (route) {
          navigate("/" + route);
        } else {
          refetch();
        }
      },
      (error) => {
        console.error("Error reading user notifications.", error);
        showErrorToast({
          errorTitle: "Error reading user notifications.",
          error,
        });
      }
    );
  }

  const [currentPage, setCurrentPage] = createSignal(1);
  const pageSize = 5;

  const sortedNotifications = createMemo(() => {
    const sorted = [...(userNotifications() || [])].sort((a, b) => {
      if (a.isRead !== b.isRead) return a.isRead ? 1 : -1;
      return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
    });
    const start = (currentPage() - 1) * pageSize;
    return sorted.slice(start, start + pageSize);
  });

  const totalPages = createMemo(() => {
    return Math.ceil((userNotifications()?.length || 0) / pageSize);
  });

  return (
    <Dialog>
      <DialogTrigger
        as={(props: DialogTriggerProps) => (
          <Button variant="ghost" size={"icon"} {...props}>
            <Bell size={18} />
          </Button>
        )}
      />
      <DialogContent class="max-w-2xl overflow-auto">
        <DialogHeader>
          <DialogTitle>Notifications</DialogTitle>
        </DialogHeader>
        <div class="grid gap-6 py-2">
          <Show
            when={sortedNotifications().length > 0}
            fallback={
              <div class="text-muted-foreground">
                You have no notifications.
              </div>
            }
          >
            <For each={sortedNotifications()}>
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
                onChange={setCurrentPage}
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
          </Show>
        </div>
      </DialogContent>
    </Dialog>
  );
}

export default UserNotificationDialog;
