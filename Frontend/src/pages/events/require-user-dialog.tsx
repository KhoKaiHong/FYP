import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogClose,
  AlertDialogContent,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { useNavigate } from "@solidjs/router";

type RequireUserDialogProps = {
  open: boolean;
  onClose: () => void;
};

function RequireUserDialog(props: RequireUserDialogProps) {

    const navigate = useNavigate();

    function performRedirect() {
        props.onClose();
        navigate("/login", { resolve: false });
    }
  return (
    <AlertDialog open={props.open}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle class="text-base font-medium">
            You must be logged in as a donor to perform this action. Proceed to
            login?
          </AlertDialogTitle>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogClose onClick={props.onClose}>Cancel</AlertDialogClose>
          <AlertDialogAction onClick={performRedirect}>Ok</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
};

export default RequireUserDialog;
