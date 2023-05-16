import { InstanceEnvType, PrismaClient } from '@prisma/client';
import { ulid } from 'ulidx';
import { tracker } from '../lib/tracker';
// import { UserId } from "./users.service";

import { db } from '../db/connection';
import { WorkspaceId, workspacesTable } from '../db/schema/workspaces.schema';
import { User, UserId, usersTable } from '../db/schema/users.schema';

import { eq } from 'drizzle-orm';

// export type WorkspaceId = string;

// this will become a model when we implement db
// export type Workspace = {
//   id: WorkspaceId;
//   instanceType: 'local' | 'private' | 'si_sass'; // only local used for now...
//   instanceUrl: string;
//   displayName: string;
//   // slug: string;
//   // currently workspaces are single player, and controlled by this prop
//   createdByUserId: UserId;
//   createdAt: ISODateTimeString;
// };

// TODO: replace all this with actual db calls...
export async function getWorkspaceById(id: WorkspaceId) {
  const results = await db.select()
    .from(workspacesTable)
    .where(eq(workspacesTable.id, id))
    .limit(1);
  return results?.[0];
  // return await prisma.workspace.findUnique({ where: { id } });
}

export async function createWorkspace(creatorUser: User) {
  const result = await db.insert(workspacesTable).values({
    id: ulid() as WorkspaceId,
    instanceEnvType: InstanceEnvType.LOCAL,
    instanceUrl: 'http://localhost:8080',
    displayName: `${creatorUser.nickname}'s dev workspace`,
    creatorUserId: creatorUser.id,
  }).returning();
  const newWorkspace = result[0];

  // tracker.trackEvent(creatorUser, 'create_workspace', {
  //   workspaceId: newWorkspace.id,
  //   // TODO: track env type and other data when it becomes useful
  // });

  return newWorkspace;
}

export async function getUserWorkspaces(userId: UserId) {
  const results = await db.select()
    .from(workspacesTable)
    .where(eq(workspacesTable.creatorUserId, userId));
  return results;
}
