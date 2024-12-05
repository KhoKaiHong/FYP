import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogContent,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
  } from "@/components/ui/alert-dialog";
  
  type EventAtCapacityDialogProps = {
    open: boolean;
    onClose: () => void;
  };
  
  function EventAtCapacityDialog(props: EventAtCapacityDialogProps) {
    return (
      <AlertDialog open={props.open}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle class="text-base font-medium">
              Event is currently full. Sorry for any inconveniences caused.
            </AlertDialogTitle>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogAction onClick={props.onClose}>Ok</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }
  
  export default EventAtCapacityDialog;
  