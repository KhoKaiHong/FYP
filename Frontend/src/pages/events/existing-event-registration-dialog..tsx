import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogContent,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
  } from "@/components/ui/alert-dialog";
  
  type ExistingEventRegistrationDialogProps = {
    open: boolean;
    onClose: () => void;
  };
  
  function ExistingEventRegistrationDialog(props: ExistingEventRegistrationDialogProps) {
    return (
      <AlertDialog open={props.open}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle class="text-base font-medium">
              You already have an existing event registration.
            </AlertDialogTitle>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogAction onClick={props.onClose}>Ok</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }
  
  export default ExistingEventRegistrationDialog;
  