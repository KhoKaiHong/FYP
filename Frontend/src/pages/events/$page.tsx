import Navbar from "@/components/navigation-bar";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useNavigate, useLocation } from "@solidjs/router";
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
import { createMemo, createSignal, For, Index } from "solid-js";
import { Portal } from "solid-js/web";
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

  const [stateIdChosen, setStateIdChosen] = createSignal(0);

  const districtsAvailable = createMemo(() => {
    const data = districts();
    const stateId = stateIdChosen();

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

  const [districtInput, setDistrictInput] = createSignal<District | null>(null);

  const [selectedDateRange, setSelectedDateRange] = createSignal<
    DateValue[] | null
  >(null);

  return (
    <div>
      <Navbar />
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 p-8">
        <div class="lg:col-span-1 space-y-4">
          <Card>
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
                <DatePickerControl>
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
                options={states() ?? []}
                optionValue="id"
                optionTextValue="name"
                disabled={states() ? false : true}
                onChange={(e) => {
                  setStateIdChosen(e?.id ?? 0);
                  setDistrictInput(null);
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
                value={districtInput()}
                onChange={(selectedOption) => {
                  setDistrictInput(selectedOption);
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
                        districtInput()?.districtName ?? "";
                    }}
                  />
                </ComboboxTrigger>
                <ComboboxContent />
              </Combobox>
              <Button class="w-full">Apply Filters</Button>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Events</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4 h-full max-h-64 overflow-auto">
              <For each={events()}>
                {(event) => (
                  <Card>
                    <CardHeader>
                      <CardTitle>{event.address}</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p>{event.currentAttendees}/{event.maxAttendees}</p>
                      <p>{event.startTime} - {event.endTime}</p>
                    </CardContent>
                  </Card>
                )}
              </For>
            </CardContent>
          </Card>
        </div>

        <div class="lg:col-span-2">
          <Card>
            <CardHeader>
              <CardTitle>Event Map</CardTitle>
            </CardHeader>
            <CardContent></CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

export default Events;
