import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogContent,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
  } from "@/components/ui/alert-dialog";
  
  type EventRegistrationSuccessfulDialogProps = {
    open: boolean;
    onClose: () => void;
  };
  
  function EventRegistrationSuccessfulDialog(props: EventRegistrationSuccessfulDialogProps) {
    return (
      <AlertDialog open={props.open}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle class="text-base font-medium">
              Event registration successful. Thank you for your contribution.
            </AlertDialogTitle>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogAction onClick={props.onClose}>Ok</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }
  
  export default EventRegistrationSuccessfulDialog;
  