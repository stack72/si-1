import { InferModel } from "drizzle-orm";
import {
  boolean, index, jsonb, pgTable, text, uniqueIndex
} from "drizzle-orm/pg-core";
import { BrandedUlid, ulid } from "./_helpers";

import { UserId, usersTable } from './users.schema';

export type WorkspaceId = BrandedUlid<'WorkspaceId'>;

export const workspacesTable = pgTable('workspaces', {
  /** id for the user */
  id: ulid('id').$type<WorkspaceId>().primaryKey(),

  /** type of instance (local, private, si sass)  */
  instanceEnvType: text("instance_env_type").notNull(),
  /** url of instance  */
  instanceUrl: text("instance_url"),
  /** label for the workspace  */
  displayName: text("display_name"),

  /** id of user who created workspace  */
  creatorUserId: ulid("creator_user_id").$type<UserId>().references(() => usersTable.id),
});

export type Workspace = InferModel<typeof workspacesTable>;
export type NewWorkspace = InferModel<typeof workspacesTable, 'insert'>;
