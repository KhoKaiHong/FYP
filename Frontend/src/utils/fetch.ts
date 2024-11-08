const BACKEND_PATH = import.meta.env.VITE_BACKEND_PATH || 'http://localhost:3001';

export async function fetchWithAuth({
  path,
  method = 'GET',
  body = null,
}: {
  path: string;
  method?: string;
  body?: any;
}) {
  // Check if access_token and refresh_token are in localStorage
  const accessToken = localStorage.getItem('access_token');
  const refreshToken = localStorage.getItem('refresh_token');

  if (!accessToken || !refreshToken) {
    // If no tokens, redirect to home page
    window.location.href = '/';
    return;
  }

  // Function to handle API calls with the access_token
  const makeRequest = async (accessToken: string) => {
    try {
      const response = await fetch(`${BACKEND_PATH}${path}`, {
        method,
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${accessToken}`,
        },
        body: body ? JSON.stringify(body) : null,
      });

      if (response.ok) {
        // Return response JSON if successful
        return await response.json();
      } else {
        // Handle specific error cases
        const errorResponse = await response.json();

        if (response.status === 401 && errorResponse.error?.message === 'ACCESS_TOKEN_EXPIRED') {
          // If access token expired, try to refresh it
          return await handleTokenRefresh(accessToken, refreshToken);
        } else if (
          (response.status === 401 && errorResponse.error?.message === 'SESSION_EXPIRED') ||
          (response.status === 403 && errorResponse.error?.message === 'NO_AUTH')
        ) {
          // If session expired or no authorization, clear tokens and redirect
          localStorage.removeItem('access_token');
          localStorage.removeItem('refresh_token');
          window.location.href = '/';
        } else {
          // For other errors, just redirect to home
          window.location.href = '/';
        }
      }
    } catch (error) {
      console.error('Error during fetch:', error);
      window.location.href = '/';
    }
  };

  // Function to handle token refresh if expired
  const handleTokenRefresh = async (accessToken: string, refreshToken: string) => {
    try {
      const refreshResponse = await fetch(`${BACKEND_PATH}/api/refresh`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${accessToken}`,
        },
        body: JSON.stringify({ refresh_token: refreshToken }),
      });

      if (refreshResponse.ok) {
        const refreshData = await refreshResponse.json();
        if (refreshData.result.success) {
          // Successfully refreshed tokens, update them in localStorage
          localStorage.setItem('access_token', refreshData.result.access_token);
          localStorage.setItem('refresh_token', refreshData.result.refresh_token);

          // Retry the original request with the new access token
          return await makeRequest(localStorage.getItem('access_token'));
        }
      }

      // If refresh failed, clear tokens and redirect
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
      window.location.href = '/';
    } catch (error) {
      console.error('Error during token refresh:', error);
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
      window.location.href = '/';
    }
  };

  // Start with the initial request using the access token
  return await makeRequest(accessToken);
}
