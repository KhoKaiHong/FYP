import { Button } from "@/components/ui/button";
import { useNavigate } from "@solidjs/router";

function NotFound() {
  const navigate = useNavigate();

  return (
    <div class="flex items-center justify-center min-h-screen bg-background">
      <div class="text-center">
        <h1 class="text-6xl font-bold text-foreground mb-4">404</h1>
        <h2 class="text-3xl font-semibold text-foreground mb-6">
          Page Not Found
        </h2>
        <p class="text-xl text-foreground mb-8">
          Oops! The page you're looking for doesn't exist.
        </p>
        <Button onClick={() => navigate("/")}>Return Home</Button>
      </div>
    </div>
  );
}

export default NotFound;
