import { customType } from "drizzle-orm/pg-core";

export const ulid = customType<{ data: string }>({
  dataType() {
    return 'char(26)';
  },
});

type Brand<K, T> = K & { __brand: T };
export type BrandedUlid<T> = Brand<string, T>;
