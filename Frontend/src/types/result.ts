import { Error } from "@/types/error";

export type ApiResult<T = any> = T | Error;