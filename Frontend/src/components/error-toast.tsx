import {
  Toast,
  ToastContent,
  ToastProgress,
  ToastTitle,
} from "@/components/ui/toast";
import { toaster } from "@kobalte/core";
import { getErrorMessage } from "@/utils/error";
import { Error } from "@/types/error";

function showErrorToast(error: Error) {
  toaster.show((props) => (
    <Toast toastId={props.toastId} variant="destructive">
      <ToastContent>
        <ToastTitle>{getErrorMessage(error)}</ToastTitle>
      </ToastContent>
      <ToastProgress />
    </Toast>
  ));
}

export default showErrorToast;
