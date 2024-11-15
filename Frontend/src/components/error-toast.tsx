import {
  Toast,
  ToastContent,
  ToastDescription,
  ToastProgress,
  ToastTitle,
} from "@/components/ui/toast";
import { toaster } from "@kobalte/core";
import { getErrorMessage } from "@/utils/error";
import { AppError } from "@/types/error";
import { Show } from "solid-js";

type ErrorToastProps = {
  errorTitle?: string;
  error: AppError;
};

function showErrorToast({
  errorTitle = "",
  error,
}: ErrorToastProps) {
  toaster.show((props) => (
    <Toast toastId={props.toastId} variant="destructive">
      <ToastContent>
        <Show when={errorTitle !== ""}>
          <ToastTitle>{errorTitle}</ToastTitle>
        </Show>
        <ToastDescription>
          {getErrorMessage(error)}
        </ToastDescription>
      </ToastContent>
      <ToastProgress />
    </Toast>
  ));
}

export default showErrorToast;
