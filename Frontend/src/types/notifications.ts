export type UserNotification = {
    id: number;
    description: string;
    redirect?: string;
    isRead: boolean;
    createdAt: string;
    userId: number;
};

export type UserNotificationsResponse = {
    data: {
        userNotifications: UserNotification[];
    };
};

export type OrganiserNotification = {
    id: number;
    description: string;
    redirect?: string;
    isRead: boolean;
    createdAt: string;
    organiserId: number;
};

export type OrganiserNotificationsResponse = {
    data: {
        organiserNotifications: OrganiserNotification[];
    };
};

export type FacilityNotification = {
    id: number;
    description: string;
    redirect?: string;
    isRead: boolean;
    createdAt: string;
    facilityId: number;
};

export type FacilityNotificationsResponse = {
    data: {
        facilityNotifications: FacilityNotification[];
    };
};

export type AdminNotification = {
    id: number;
    description: string;
    redirect?: string;
    isRead: boolean;
    createdAt: string;
    adminId: number;
};

export type AdminNotificationsResponse = {
    data: {
        adminNotifications: AdminNotification[];
    };
};

export type ReadNotificationPayload = {
    notificationId: number;
};

export type ReadNotificationResponse = {
    data: {
        success: boolean;
    };
};
