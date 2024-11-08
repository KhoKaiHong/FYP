const BACKEND_PATH = import.meta.env.VITE_BACKEND_PATH || 'http://localhost:3001';

/**
 * Logs in a user with ic_number and password.
 * Saves access_token and refresh_token to localStorage on success.
 */
export async function userLogin(ic_number: string, password: string) {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/userlogin`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ ic_number, password }),
    });

    const result = await response.json();

    if (response.ok && result.result.success) {
      localStorage.setItem('access_token', result.result.access_token);
      localStorage.setItem('refresh_token', result.result.refresh_token);
      return result;
    } else {
      throw new Error(result.message || 'User login failed');
    }
  } catch (error) {
    console.error('Error during user login:', error);
    throw error;
  }
}

/**
 * Logs in a facility with email and password.
 * Saves access_token and refresh_token to localStorage on success.
 */
export async function facilityLogin(email: string, password: string) {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/facilitylogin`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ email, password }),
    });

    const result = await response.json();

    if (response.ok && result.result.success) {
      localStorage.setItem('access_token', result.result.access_token);
      localStorage.setItem('refresh_token', result.result.refresh_token);
      return result;
    } else {
      throw new Error(result.message || 'Facility login failed');
    }
  } catch (error) {
    console.error('Error during facility login:', error);
    throw error;
  }
}

/**
 * Logs in an organiser with email and password.
 * Saves access_token and refresh_token to localStorage on success.
 */
export async function organiserLogin(email: string, password: string) {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/organiserlogin`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ email, password }),
    });

    const result = await response.json();

    if (response.ok && result.result.success) {
      localStorage.setItem('access_token', result.result.access_token);
      localStorage.setItem('refresh_token', result.result.refresh_token);
      return result;
    } else {
      throw new Error(result.message || 'Organiser login failed');
    }
  } catch (error) {
    console.error('Error during organiser login:', error);
    throw error;
  }
}

/**
 * Logs in an admin with email and password.
 * Saves access_token and refresh_token to localStorage on success.
 */
export async function adminLogin(email: string, password: string) {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/adminlogin`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ email, password }),
    });

    const result = await response.json();

    if (response.ok && result.result.success) {
      localStorage.setItem('access_token', result.result.access_token);
      localStorage.setItem('refresh_token', result.result.refresh_token);
      return result;
    } else {
      throw new Error(result.message || 'Admin login failed');
    }
  } catch (error) {
    console.error('Error during admin login:', error);
    throw error;
  }
}
