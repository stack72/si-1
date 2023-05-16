import { InferModel } from "drizzle-orm";
import {
  boolean, index, jsonb, pgTable, text, uniqueIndex
} from "drizzle-orm/pg-core";
import { BrandedUlid, ulid } from "./_helpers";

export type UserId = BrandedUlid<'UserId'>;

export const usersTable = pgTable('users', {
  /** id for the user */
  id: ulid('id').$type<UserId>().primaryKey(),
  /** auth0's primary key, based on auth provider and their id */
  auth0Id: text('auth_0_id').notNull(),
  /** raw json blob of Auth0 data  */
  auth0Details: jsonb('auth0_details').$type(),
  /** single name string we can use as label for the user  */
  nickname: text('nickname').notNull(),
  /** user's email  */
  email: text('email').notNull(),
  /** whether email has been verified  */
  emailVerified: boolean('email_verified').default(false),
  /** user's first name  */
  firstName: text('first_name'),
  /** user's last name  */
  lastName: text("last_name"),
  /** public url to profile photo  */
  pictureUrl: text("picture_url"),

  /** user's discord username/tag - ex: coolbeans#1234  */
  discordUsername: text("discord_username"),
  /** user's github username  */
  githubUsername: text("github_username"),
  /** data about where user is in onboarding  */
  onboardingDetails: jsonb("onboarding_details").$type(),

  // /** array of workspaces the user created  */
  // CreatedWorkspaces Workspace[]
  // TosAgreement      TosAgreement[]
}, (users) => ({
  emailIdx: index().on(users.email),
  uniqueAuth0IdIndex: uniqueIndex().on(users.auth0Id), // unique index
}));

export type User = InferModel<typeof usersTable>;
export type NewUser = InferModel<typeof usersTable, 'insert'>;
