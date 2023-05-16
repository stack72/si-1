import { InferModel } from "drizzle-orm";
import {
  boolean, index, jsonb, pgTable, text, timestamp, uniqueIndex
} from "drizzle-orm/pg-core";
import { BrandedUlid, ulid } from "./_helpers";

import { UserId, usersTable } from './users.schema';

export type TosAgreementId = BrandedUlid<'TosAgreementId'>;

export const tosAgreementsTable = pgTable('tos_agreements', {
  /** id of agreement - not really used for anything... */
  id: ulid('id').$type<TosAgreementId>().primaryKey(),
  userId: ulid('user_id').notNull().$type<UserId>().references(() => usersTable.id),
  /** TOS version ID agreed to (these are sortable to find latest)  */
  tosVersionId: text('tos_version_id').notNull(),
  /** timestamp when they agreed to the TOS  */
  timestamp: timestamp('timestamp').defaultNow().notNull(),
  ipAddress: text('ip_address').notNull(),
}, (tosAgreements) => ({
  userIdx: index().on(tosAgreements.userId),
}));

export type TosAgreement = InferModel<typeof tosAgreementsTable>;
export type NewTosAgreement = InferModel<typeof tosAgreementsTable, 'insert'>;

