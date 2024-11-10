import { ApiResult } from "@/types/result";
import {
  UserLoginResponse,
  FacilityLoginResponse,
  OrganiserLoginResponse,
  AdminLoginResponse,
} from "@/types/login";
import { parseErrorResponse } from "@/utils/error";

const BACKEND_PATH =
  import.meta.env.VITE_BACKEND_PATH || "http://localhost:3001";

export async function userLogin(
  icNumber: string,
  password: string
): Promise<ApiResult<UserLoginResponse>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/userlogin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ icNumber, password }),
    });

    const result = await response.json();

    if (response.ok) {
      localStorage.setItem("accessToken", result.data.accessToken);
      localStorage.setItem("refreshToken", result.data.refreshToken);
      return result as UserLoginResponse;
    }
    
    const parsedError = parseErrorResponse(result);
    return parsedError;

  } catch (error) {
    console.error("Error during user login:", error);
    return { error: "UNKNOWN_ERROR" };
  }
}

// Updated facilityLogin function with ApiResult<FacilityLoginResponse>
export async function facilityLogin(
  email: string,
  password: string
): Promise<ApiResult<FacilityLoginResponse>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/facilitylogin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    });

    const result = await response.json();

    if (response.ok) {
      localStorage.setItem("accessToken", result.data.accessToken);
      localStorage.setItem("refreshToken", result.data.refreshToken);
      return result as FacilityLoginResponse;
    }
    
    const parsedError = parseErrorResponse(result);
    return parsedError;

  } catch (error) {
    console.error("Error during facility login:", error);
    return { error: "UNKNOWN_ERROR" };
  }
}

// Updated organiserLogin function with ApiResult<OrganiserLoginResponse>
export async function organiserLogin(
  email: string,
  password: string
): Promise<ApiResult<OrganiserLoginResponse>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/organiserlogin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    });

    const result = await response.json();

    if (response.ok) {
      localStorage.setItem("accessToken", result.data.accessToken);
      localStorage.setItem("refreshToken", result.data.refreshToken);
      return result as OrganiserLoginResponse;
    }
    
    const parsedError = parseErrorResponse(result);
    return parsedError;

  } catch (error) {
    console.error("Error during organiser login:", error);
    return { error: "SERVICE_ERROR" };
  }
}

// Updated adminLogin function with ApiResult<AdminLoginResponse>
export async function adminLogin(
  email: string,
  password: string
): Promise<ApiResult<AdminLoginResponse>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/adminlogin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    });

    const result = await response.json();

    if (response.ok) {
      localStorage.setItem("accessToken", result.data.accessToken);
      localStorage.setItem("refreshToken", result.data.refreshToken);
      return result as AdminLoginResponse;
    }
    
    const parsedError = parseErrorResponse(result);
    return parsedError;

  } catch (error) {
    console.error("Error during admin login:", error);
    return { error: "SERVICE_ERROR" };
  }
}
