import {
    Toast,
    ToastContent,
    ToastDescription,
    ToastProgress,
    ToastTitle,
  } from "@/components/ui/toast";
  import { toaster } from "@kobalte/core";
  import { Show } from "solid-js";
  
  type SuccessToastProps = {
    successTitle?: string;
    successDescription?: string;
  };
  
  function showSuccessToast({
    successTitle = "",
    successDescription = "",
  }: SuccessToastProps) {
    toaster.show((props) => (
      <Toast toastId={props.toastId} variant="success">
        <ToastContent>
          <Show when={successTitle !== ""}>
            <ToastTitle>{successTitle}</ToastTitle>
          </Show>
          <Show when={successDescription !== ""}>
            <ToastDescription>{successDescription}</ToastDescription>
          </Show>
        </ToastContent>
        <ToastProgress />
      </Toast>
    ));
  }
  
  export default showSuccessToast;
  