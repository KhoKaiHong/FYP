import type { AlertDialogTriggerProps } from "@kobalte/core/alert-dialog";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogClose,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import { useUser } from "@/context/user-context";

function RegisterRedirectDialog() {
  const { user, setUser, role, setRole, isAuthenticated, setIsAuthenticated } =
    useUser();

  return (
    <AlertDialog open={isAuthenticated()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogDescription>
            You are already logged in. Do you wish to log out? Pressing no will
            redirect you to the home page.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogClose>Yes</AlertDialogClose>
          <AlertDialogAction>No</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

export default RegisterRedirectDialog;
