/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type BrineDB = BrineDb
export class BrineDb {
  connectionUri: string
  constructor(connectionUri: string)
  connect(): boolean
  migrate(): void
  get(key: string): string | null
  set(key: string, value: string): void
  setMany(data: Array<[string, string]>): void
  getMany(keys: Array<string>): Record<string, string | undefined | null>
  clear(): void
  delete(key: string): void
  deleteMany(keys: Array<string>): void
  keys(): Array<string>
  values(): Array<string>
  count(): number
  has(key: string): boolean
  close(): void
}
