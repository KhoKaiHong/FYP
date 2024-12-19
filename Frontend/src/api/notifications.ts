import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import {
  UserNotificationsResponse,
  OrganiserNotificationsResponse,
  FacilityNotificationsResponse,
  AdminNotificationsResponse,
  ReadNotificationPayload,
  ReadNotificationResponse,
} from "@/types/notifications";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function listUserNotifications(): Promise<
  Result<UserNotificationsResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<UserNotificationsResponse>({
      path: "/api/user-notifications",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as UserNotificationsResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing user notifications:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function readUserNotification(
  readUserNotificationPayload: ReadNotificationPayload
): Promise<Result<ReadNotificationResponse, AppError>> {
  try {
    const result = await fetchWithAuth<ReadNotificationResponse>({
      path: "/api/user-notification",
      method: "PATCH",
      body: JSON.stringify(readUserNotificationPayload),
    });

    if (result.isOk()) {
      return ok(result.value as ReadNotificationResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when reading user notification:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function listOrganiserNotifications(): Promise<
  Result<OrganiserNotificationsResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<OrganiserNotificationsResponse>({
      path: "/api/organiser-notifications",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as OrganiserNotificationsResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing organiser notifications:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function readOrganiserNotification(
  readOrganiserNotificationPayload: ReadNotificationPayload
): Promise<Result<ReadNotificationResponse, AppError>> {
  try {
    const result = await fetchWithAuth<ReadNotificationResponse>({
      path: "/api/organiser-notification",
      method: "PATCH",
      body: JSON.stringify(readOrganiserNotificationPayload),
    });

    if (result.isOk()) {
      return ok(result.value as ReadNotificationResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when reading organiser notification:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function listFacilityNotifications(): Promise<
  Result<FacilityNotificationsResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<FacilityNotificationsResponse>({
      path: "/api/facility-notifications",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as FacilityNotificationsResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing facility notifications:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function readFacilityNotification(
  readFacilityNotificationPayload: ReadNotificationPayload
): Promise<Result<ReadNotificationResponse, AppError>> {
  try {
    const result = await fetchWithAuth<ReadNotificationResponse>({
      path: "/api/facility-notification",
      method: "PATCH",
      body: JSON.stringify(readFacilityNotificationPayload),
    });

    if (result.isOk()) {
      return ok(result.value as ReadNotificationResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when reading facility notification:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function listAdminNotifications(): Promise<
  Result<AdminNotificationsResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<AdminNotificationsResponse>({
      path: "/api/admin-notifications",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as AdminNotificationsResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing admin notifications:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function readAdminNotification(
  readAdminNotificationPayload: ReadNotificationPayload
): Promise<Result<ReadNotificationResponse, AppError>> {
  try {
    const result = await fetchWithAuth<ReadNotificationResponse>({
      path: "/api/admin-notification",
      method: "PATCH",
      body: JSON.stringify(readAdminNotificationPayload),
    });

    if (result.isOk()) {
      return ok(result.value as ReadNotificationResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when reading admin notification:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
