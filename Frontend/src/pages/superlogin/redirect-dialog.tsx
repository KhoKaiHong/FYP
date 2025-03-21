import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogTitle,
  AlertDialogFooter,
  AlertDialogHeader,
} from "@/components/ui/alert-dialog";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { Button } from "@/components/ui/button";

function RegisterRedirectDialog() {
  const { isAuthenticated, logout } = useUser();

  const navigate = useNavigate();

  return (
    <AlertDialog open={isAuthenticated()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle class="text-base font-medium">
            You are already logged in. Do you wish to log out? <br />
            Pressing no will redirect you to the home page.
          </AlertDialogTitle>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <Button variant="destructive" onClick={async () => await logout()}>
            Yes
          </Button>
          <Button onClick={() => navigate("/", { resolve: false })}>No</Button>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

export default RegisterRedirectDialog;
