import { ColumnDef } from "@tanstack/solid-table"
import { NewEventProposal } from "@/types/new-event-proposal"
 
export const columns: ColumnDef<NewEventProposal>[] = [
  {
    accessorKey: "id",
    header: "ID"
  },
  {
    accessorKey: "location",
    header: "Location"
  },
  {
    accessorKey: "address",
    header: "Address"
  },
  {
    accessorKey: "startTime",
    header: "Start Time"
  },
  {
    accessorKey: "endTime",
    header: "End Time"
  },
  {
    accessorKey: "maxAttendees",
    header: "Max Attendees"
  },
  {
    accessorKey: "status",
    header: "Status"
  },
  {
    accessorKey: "rejectionReason",
    header: "Rejection Reason"
  },
  {
    accessorKey: "stateName",
    header: "State"
  },
  {
    accessorKey: "districtName",
    header: "District"
  },
  {
    accessorKey: "organiserName",
    header: "Organiser"
  }
]

// export type NewEventProposal = {
//     id: number;
//     location: string;
//     address: string;
//     startTime: string;
//     endTime: string;
//     maxAttendees: number;
//     latitude: number;
//     longitude: number;
//     status: string;
//     rejectionReason: string | null;
//     facilityId: number;
//     facilityEmail: string;
//     facilityName: string;
//     facilityAddress: string;
//     facilityPhoneNumber: string;
//     organiserId: number;
//     organiserEmail: string;
//     organiserName: string;
//     organiserPhoneNumber: string;
//     stateId: number;
//     stateName: string;
//     districtId: number;
//     districtName: string;
//   };