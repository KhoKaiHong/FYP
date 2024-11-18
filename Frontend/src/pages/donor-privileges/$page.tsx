import Navbar from "@/components/navigation-bar";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { A } from "@solidjs/router";
import { Bed, Syringe, Cross } from "lucide-solid";

function DonorPrivileges() {
  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-8">
        <div class="space-y-8 lg:grid lg:grid-cols-2 lg:gap-8 lg:space-y-0">
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                1 - 5 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>Second class wards for 4 months</p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                2 times (within 12 months)
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>Second class wards for 4 months</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Syringe class="shrink-0" />
                <p>Free Hepatitis B vaccination</p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                6 - 10 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>Second class wards for 6 months</p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                11 - 15 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>Second class wards for 12 months</p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                16 - 20 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>Second class wards for 24 months</p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                21 - 30 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>Second class wards for 3 years</p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                31 - 40 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>First class wards for 4 years</p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                41 - 50 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>First class wards for 6 years</p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                51 - 75 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>
                  First class wards for 10 years, lifelong second class wards
                  after that
                </p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                76 - 100 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>
                  First class wards for 15 years, lifelong second class wards
                  after that
                </p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader class="bg-brand rounded-t-xl">
              <CardTitle class="text-slate-50 text-xl tracking-wide">
                More than 100 times
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <div class="flex gap-x-2 items-center">
                <Cross class="shrink-0" />
                <p>Free outpatient, dental and medical treatment</p>
              </div>
              <div class="flex gap-x-2 items-center">
                <Bed class="shrink-0" />
                <p>
                  First class wards for 20 years, lifelong second class wards
                  after that
                </p>
              </div>
            </CardContent>
          </Card>
        </div>
        <div class="text-muted-foreground space-y-2">
          <p>
            * Donors need to show blood donation records to the hospital to be able to enjoy these privileges.
          </p>
          <p>
            * Privilege eligibility is subject to the availability of beds in
            the ward.
          </p>
          <p>
            * These privileges can only be enjoyed by blood donors who are
            Malaysian citizens only.
          </p>
          <p>
            * The stated privilege period is valid starting from the date of last
            donation.
          </p>
          <p>
            This data is brought to you by the{" "}
            <A
              href="https://pdn.gov.my/v2/keistimewaan-penderma//"
              target="_blank"
              class="text-blue-500 hover:underline hover:text-blue-600"
            >
              National Blood Centre
            </A>
            .
          </p>
        </div>
      </div>
    </div>
  );
}

export default DonorPrivileges;
