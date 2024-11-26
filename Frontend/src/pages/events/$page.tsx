import Navbar from "@/components/navigation-bar";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { A } from "@solidjs/router";
import {
  DatePicker,
  DatePickerContent,
  DatePickerContext,
  DatePickerControl,
  DatePickerInput,
  DatePickerPositioner,
  DatePickerRangeText,
  DatePickerTable,
  DatePickerTableBody,
  DatePickerTableCell,
  DatePickerTableCellTrigger,
  DatePickerTableHead,
  DatePickerTableHeader,
  DatePickerTableRow,
  DatePickerTrigger,
  DatePickerView,
  DatePickerViewControl,
  DatePickerViewTrigger,
} from "@/components/ui/date-picker";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  createEffect,
  createMemo,
  createSignal,
  For,
  Index,
  onMount,
} from "solid-js";
import { Portal } from "solid-js/web";
import { Event } from "@/types/events";
import { listDistricts } from "@/api/districts";
import { listFutureEvents } from "@/api/events";
import { State } from "@/types/states";
import showErrorToast from "@/components/error-toast";
import { createResource } from "solid-js";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import {
  Combobox,
  ComboboxContent,
  ComboboxInput,
  ComboboxItem,
  ComboboxTrigger,
} from "@/components/ui/combobox";
import { District } from "@/types/districts";
import { today } from "@internationalized/date";
import { DateValue } from "@ark-ui/solid";
import { MapPin, Calendar, Clock, UsersRound } from "lucide-solid";
import { DialogTriggerProps } from "@kobalte/core/dialog";

function Events() {
  async function getEventsData() {
    try {
      const [districtsResponse, eventsResponse] = await Promise.all([
        listDistricts(),
        listFutureEvents(),
      ]);

      const statesAndDistricts = districtsResponse.match(
        (data) => {
          const districts = data.data.districts;
          const statesMap = districts.reduce((map, district) => {
            map.set(district.stateId, district.stateName);
            return map;
          }, new Map<number, string>());

          const states = Array.from(statesMap, ([id, name]) => ({
            id,
            name,
          })) as State[];

          return { states, districts };
        },
        (error) => {
          console.error("Error fetching states and districts.", error);
          return null;
        }
      );

      const events = eventsResponse.match(
        (data) => data.data.events,
        (error) => {
          console.error("Error fetching events.", error);
          return null;
        }
      );

      if (!events || !statesAndDistricts) {
        showErrorToast({
          errorTitle: "Error loading events data.",
          error: { message: "SERVICE_ERROR" },
        });
        return null;
      }

      return {
        events,
        states: statesAndDistricts.states,
        districts: statesAndDistricts.districts,
      };
    } catch (err) {
      console.error("Error loading events: ", err);
      showErrorToast({
        errorTitle: "Error loading events.",
        error: { message: "UNKNOWN_ERROR" },
      });
      return null;
    }
  }

  const [eventsData] = createResource(getEventsData);
  const events = () => eventsData()?.events ?? null;
  const states = () => eventsData()?.states ?? null;
  const districts = () => eventsData()?.districts ?? null;

  let map: google.maps.Map | undefined;
  let infoWindow: google.maps.InfoWindow | undefined;
  const markers: google.maps.marker.AdvancedMarkerElement[] = [];

  onMount(async () => {
    const { Map, InfoWindow } = (await google.maps.importLibrary(
      "maps"
    )) as google.maps.MapsLibrary;

    map = new Map(document.getElementById("map") as HTMLElement, {
      center: { lat: 3.1732962387784367, lng: 101.70668106095312 },
      zoom: 10,
      mapId: "f7a7a21c7ed4070e",
    });

    infoWindow = new InfoWindow();
  });

  createEffect(async () => {
    const eventsPin = events();

    if (map && infoWindow && eventsPin && eventsPin.length > 0) {
      const { AdvancedMarkerElement } = (await google.maps.importLibrary(
        "marker"
      )) as google.maps.MarkerLibrary;

      eventsPin.forEach((event) => {
        const marker = new AdvancedMarkerElement({
          map,
          position: { lat: event.latitude, lng: event.longitude },
          title: event.address,
          gmpClickable: true,
        });

        marker.addListener("click", () => {
          infoWindow?.close();
          infoWindow?.setContent(`<p class="text-slate-600">${marker.title}</p>`);
          infoWindow?.open(marker.map, marker);
        });

        markers.push(marker);
      });
    }
  });

  const [stateChosen, setStateChosen] = createSignal<State | null>(null);

  const districtsAvailable = createMemo(() => {
    const data = districts();
    const stateId = stateChosen()?.id ?? 0;

    if (!data || stateId === 0) {
      return [];
    } else {
      const districtsInState = data.filter(
        (district) => district.stateId === stateId
      );

      return districtsInState;
    }
  });

  const districtInputDisabled = createMemo(() => {
    return !districts() || districtsAvailable().length === 0;
  });

  const [districtChosen, setDistrictChosen] = createSignal<District | null>(
    null
  );

  const [selectedDateRange, setSelectedDateRange] = createSignal<
    DateValue[] | null
  >(null);

  const [filteredEvents, setFilteredEvents] = createSignal<Event[]>([]);

  createEffect(() => {
    setFilteredEvents(events() ?? []);
  });

  async function filterEvents() {
    const allEvents = events();
    const selectedState = stateChosen();
    const selectedDistrict = districtChosen();
    const dateRange = selectedDateRange();

    if (!allEvents) return []; // Return an empty array if no events data is available

    const filteredEvents = allEvents.filter((event) => {
      const eventStartTime = new Date(event.startTime);
      const eventEndTime = new Date(event.endTime);

      function endOfDay(date: Date): Date {
        const modifiedDate = new Date(date);
        modifiedDate.setHours(23, 59, 59, 999);
        return modifiedDate;
      }

      const matchesState = !selectedState || event.stateId === selectedState.id;
      const matchesDistrict =
        !selectedDistrict || event.districtId === selectedDistrict.districtId;
      const matchesDateRange =
        !dateRange ||
        dateRange.length !== 2 ||
        (dateRange[0] &&
          dateRange[1] &&
          eventStartTime >= dateRange[0].toDate("Asia/Kuala_Lumpur") &&
          eventEndTime <= endOfDay(dateRange[1].toDate("Asia/Kuala_Lumpur")));

      return matchesState && matchesDistrict && matchesDateRange;
    });

    setFilteredEvents(filteredEvents);

    markers.forEach((marker) => (marker.map = null));
    markers.splice(0, markers.length);

    const { AdvancedMarkerElement } = (await google.maps.importLibrary(
      "marker"
    )) as google.maps.MarkerLibrary;

    // Add markers for the filtered events
    if (map && filteredEvents.length > 0) {
      filteredEvents.forEach((event) => {
        const marker = new AdvancedMarkerElement({
          map,
          position: { lat: event.latitude, lng: event.longitude },
          title: event.address,
          gmpClickable: true,
        });

        marker.addListener("click", () => {
          infoWindow?.close();
          infoWindow?.setContent(`<p class="text-slate-600">${marker.title}</p>`);
          infoWindow?.open(marker.map, marker);
        });

        markers.push(marker);
      });
    }
  }

  function expandMarkerPin(event: Event) {
    markers.forEach((marker) => {
      if (marker.title === event.address) {
        map?.setCenter({ lat: event.latitude, lng: event.longitude });
        map?.setZoom(18);
        infoWindow?.setContent(marker.title);
        infoWindow?.open(marker.map, marker);
        return;
      }
    });
  }

  return (
    <div>
      <Navbar />
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 p-8">
        <div class="lg:col-span-1 space-y-8">
          <Card class="border-brand border-2">
            <CardHeader>
              <CardTitle>Filters</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4">
              <DatePicker
                numOfMonths={2}
                selectionMode="range"
                locale="en-GB"
                min={today("Asia/Kuala_Lumpur") as unknown as DateValue}
                timeZone="Asia/Kuala_Lumpur"
                onValueChange={(value) => {
                  setSelectedDateRange(value.value);
                }}
              >
                <DatePickerControl class="w-full">
                  <DatePickerInput index={0} />
                  <DatePickerInput index={1} />
                  <DatePickerTrigger />
                </DatePickerControl>
                <Portal>
                  <DatePickerPositioner>
                    <DatePickerContent>
                      <DatePickerView view="day">
                        <DatePickerContext>
                          {(context) => {
                            const offset = createMemo(() =>
                              context().getOffset({ months: 1 })
                            );

                            return (
                              <>
                                <DatePickerViewControl>
                                  <DatePickerViewTrigger>
                                    <DatePickerRangeText />
                                  </DatePickerViewTrigger>
                                </DatePickerViewControl>
                                <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
                                  <DatePickerTable>
                                    <DatePickerTableHead>
                                      <DatePickerTableRow>
                                        <Index each={context().weekDays}>
                                          {(weekDay) => (
                                            <DatePickerTableHeader>
                                              {weekDay().short}
                                            </DatePickerTableHeader>
                                          )}
                                        </Index>
                                      </DatePickerTableRow>
                                    </DatePickerTableHead>
                                    <DatePickerTableBody>
                                      <Index each={context().weeks}>
                                        {(week) => (
                                          <DatePickerTableRow>
                                            <Index each={week()}>
                                              {(day) => (
                                                <DatePickerTableCell
                                                  value={day()}
                                                >
                                                  <DatePickerTableCellTrigger>
                                                    {day().day}
                                                  </DatePickerTableCellTrigger>
                                                </DatePickerTableCell>
                                              )}
                                            </Index>
                                          </DatePickerTableRow>
                                        )}
                                      </Index>
                                    </DatePickerTableBody>
                                  </DatePickerTable>
                                  <DatePickerTable>
                                    <DatePickerTableHead>
                                      <DatePickerTableRow>
                                        <Index each={context().weekDays}>
                                          {(weekDay) => (
                                            <DatePickerTableHeader>
                                              {weekDay().short}
                                            </DatePickerTableHeader>
                                          )}
                                        </Index>
                                      </DatePickerTableRow>
                                    </DatePickerTableHead>
                                    <DatePickerTableBody>
                                      <Index each={offset().weeks}>
                                        {(week) => (
                                          <DatePickerTableRow>
                                            <Index each={week()}>
                                              {(day) => (
                                                <DatePickerTableCell
                                                  value={day()}
                                                  visibleRange={
                                                    offset().visibleRange
                                                  }
                                                >
                                                  <DatePickerTableCellTrigger>
                                                    {day().day}
                                                  </DatePickerTableCellTrigger>
                                                </DatePickerTableCell>
                                              )}
                                            </Index>
                                          </DatePickerTableRow>
                                        )}
                                      </Index>
                                    </DatePickerTableBody>
                                  </DatePickerTable>
                                </div>
                              </>
                            );
                          }}
                        </DatePickerContext>
                      </DatePickerView>
                      <DatePickerView view="month">
                        <DatePickerContext>
                          {(context) => (
                            <>
                              <DatePickerViewControl>
                                <DatePickerViewTrigger>
                                  <DatePickerRangeText />
                                </DatePickerViewTrigger>
                              </DatePickerViewControl>
                              <DatePickerTable>
                                <DatePickerTableBody>
                                  <Index
                                    each={context().getMonthsGrid({
                                      columns: 4,
                                      format: "short",
                                    })}
                                  >
                                    {(months) => (
                                      <DatePickerTableRow>
                                        <Index each={months()}>
                                          {(month) => (
                                            <DatePickerTableCell
                                              value={month().value}
                                            >
                                              <DatePickerTableCellTrigger>
                                                {month().label}
                                              </DatePickerTableCellTrigger>
                                            </DatePickerTableCell>
                                          )}
                                        </Index>
                                      </DatePickerTableRow>
                                    )}
                                  </Index>
                                </DatePickerTableBody>
                              </DatePickerTable>
                            </>
                          )}
                        </DatePickerContext>
                      </DatePickerView>
                      <DatePickerView view="year">
                        <DatePickerContext>
                          {(context) => (
                            <>
                              <DatePickerViewControl>
                                <DatePickerViewTrigger>
                                  <DatePickerRangeText />
                                </DatePickerViewTrigger>
                              </DatePickerViewControl>
                              <DatePickerTable>
                                <DatePickerTableBody>
                                  <Index
                                    each={context().getYearsGrid({
                                      columns: 4,
                                    })}
                                  >
                                    {(years) => (
                                      <DatePickerTableRow>
                                        <Index each={years()}>
                                          {(year) => (
                                            <DatePickerTableCell
                                              value={year().value}
                                            >
                                              <DatePickerTableCellTrigger>
                                                {year().label}
                                              </DatePickerTableCellTrigger>
                                            </DatePickerTableCell>
                                          )}
                                        </Index>
                                      </DatePickerTableRow>
                                    )}
                                  </Index>
                                </DatePickerTableBody>
                              </DatePickerTable>
                            </>
                          )}
                        </DatePickerContext>
                      </DatePickerView>
                    </DatePickerContent>
                  </DatePickerPositioner>
                </Portal>
              </DatePicker>
              <Select
                class="space-y-1 w-full"
                value={stateChosen()}
                options={states() ?? []}
                optionValue="id"
                optionTextValue="name"
                disabled={states() ? false : true}
                onChange={(e) => {
                  setStateChosen(e);
                  setDistrictChosen(null);
                }}
                placeholder="Select a state..."
                itemComponent={(props) => (
                  <SelectItem item={props.item}>
                    {props.item.rawValue.name}
                  </SelectItem>
                )}
              >
                <SelectTrigger class="w-full">
                  <SelectValue<State> class="data-[placeholder-shown]:text-muted-foreground">
                    {(state) => state.selectedOption().name}
                  </SelectValue>
                </SelectTrigger>
                <SelectContent />
              </Select>
              <Combobox
                class="space-y-1 w-full"
                options={districtsAvailable()}
                value={districtChosen()}
                onChange={(selectedOption) => {
                  setDistrictChosen(selectedOption);
                }}
                optionValue="districtId"
                optionTextValue="districtName"
                optionLabel="districtName"
                disabled={districtInputDisabled()}
                placeholder="Enter district..."
                itemComponent={(props) => (
                  <ComboboxItem item={props.item}>
                    {props.item.rawValue.districtName}
                  </ComboboxItem>
                )}
              >
                <ComboboxTrigger>
                  <ComboboxInput
                    onBlur={(e) => {
                      e.currentTarget.value =
                        districtChosen()?.districtName ?? "";
                    }}
                  />
                </ComboboxTrigger>
                <ComboboxContent />
              </Combobox>
              <Button class="w-full" onClick={filterEvents}>
                Apply Filters
              </Button>
            </CardContent>
          </Card>

          <Card class="border-brand border-2">
            <CardHeader>
              <CardTitle>Events</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4 h-full max-h-64 overflow-auto overscroll-contain">
              <For
                each={filteredEvents()}
                fallback={<p class="text-muted-foreground">No events found</p>}
              >
                {(event) => (
                  <Card class="border-brand">
                    <CardHeader>
                      <CardTitle>
                        <div class="flex gap-x-2 items-center">
                          <MapPin size={20} class="shrink-0" />
                          <p>{event.address}</p>
                        </div>
                      </CardTitle>
                    </CardHeader>
                    <CardContent class="space-y-2">
                      <div class="flex gap-x-2 items-center text-muted-foreground">
                        <Calendar size={20} class="shrink-0" />
                        <p>
                          {new Date(event.startTime).toLocaleDateString(
                            "en-GB",
                            {
                              timeZone: "Asia/Kuala_Lumpur",
                            }
                          )}
                        </p>
                      </div>
                      <div class="flex gap-x-2 items-center text-muted-foreground">
                        <Clock size={20} class="shrink-0" />
                        <p>
                          {new Date(event.startTime).toLocaleTimeString(
                            "en-GB",
                            {
                              timeZone: "Asia/Kuala_Lumpur",
                              hour: "2-digit",
                              minute: "2-digit",
                              hour12: true,
                            }
                          )}{" "}
                          -{" "}
                          {new Date(event.endTime).toLocaleTimeString("en-GB", {
                            timeZone: "Asia/Kuala_Lumpur",
                            hour: "2-digit",
                            minute: "2-digit",
                            hour12: true,
                          })}
                        </p>
                      </div>
                      <div class="flex gap-x-2 items-center text-muted-foreground">
                        <UsersRound size={20} class="shrink-0" />
                        <p>
                          {event.currentAttendees}/{event.maxAttendees}
                        </p>
                      </div>
                    </CardContent>
                    <CardFooter class="grid grid-cols-2 gap-x-4 gap-y-4">
                      <Dialog>
                        <DialogTrigger
                          as={(props: DialogTriggerProps) => (
                            <Button variant="outline" {...props}>
                              Details
                            </Button>
                          )}
                        />
                        <DialogContent>
                          <DialogHeader>
                            <DialogTitle>Details</DialogTitle>
                            <div class="space-y-2">
                              <p>Address: {event.address}</p>
                              <p>
                                Date:{" "}
                                {new Date(event.startTime).toLocaleDateString(
                                  "en-GB",
                                  {
                                    timeZone: "Asia/Kuala_Lumpur",
                                  }
                                )}
                              </p>
                              <p>
                                Time:{" "}
                                {new Date(event.startTime).toLocaleTimeString(
                                  "en-GB",
                                  {
                                    timeZone: "Asia/Kuala_Lumpur",
                                    hour: "2-digit",
                                    minute: "2-digit",
                                    hour12: true,
                                  }
                                )}{" "}
                                -{" "}
                                {new Date(event.endTime).toLocaleTimeString(
                                  "en-GB",
                                  {
                                    timeZone: "Asia/Kuala_Lumpur",
                                    hour: "2-digit",
                                    minute: "2-digit",
                                    hour12: true,
                                  }
                                )}
                              </p>
                              <p>
                                Attendees: {event.currentAttendees}/
                                {event.maxAttendees}
                              </p>
                              <p>Facility: {event.facilityName}</p>
                              <p>Facility Email: {event.facilityEmail}</p>
                              <p>
                                Facility Phone Number:{" "}
                                {event.facilityPhoneNumber}
                              </p>
                              <p>Facility Address: {event.facilityAddress}</p>
                              <p>Organiser: {event.organiserName}</p>
                              <p>Organiser Email: {event.organiserEmail}</p>
                              <p>
                                Organiser Phone Number:{" "}
                                {event.organiserPhoneNumber}
                              </p>
                            </div>
                          </DialogHeader>
                        </DialogContent>
                      </Dialog>
                      <Button
                        variant={"outline"}
                        onClick={() => expandMarkerPin(event)}
                      >
                        Show on Map
                      </Button>
                      <A
                        href={`https://www.google.com/maps/dir/?api=1&destination=${event.latitude},${event.longitude}&travelmode=driving`}
                        target="_blank"
                      >
                        <Button variant={"outline"} class="w-full">
                          Directions
                        </Button>
                      </A>
                      <Button
                        disabled={event.currentAttendees >= event.maxAttendees}
                      >
                        Register
                      </Button>
                    </CardFooter>
                  </Card>
                )}
              </For>
            </CardContent>
          </Card>
        </div>

        <div class="lg:col-span-2">
          <Card class="border-brand border-2">
            <CardHeader>
              <CardTitle>Event Map</CardTitle>
            </CardHeader>
            <CardContent class="flex h-[572px]">
              <div
                class="w-full h-full"
                id="map"
                ref={map as unknown as HTMLDivElement}
              ></div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

export default Events;
