import Bottle from "bottlejs";
import { ApiResponse, SDF } from "@/api/sdf";
import { ChangeSet } from "@/api/sdf/dal/change_set";
import { EditSession } from "@/api/sdf/dal/edit_session";
import { changeSet$ } from "@/observable/change_set";
import { editSession$ } from "@/observable/edit_session";
import { editMode$ } from "@/observable/edit_mode";
import { Observable, tap } from "rxjs";

interface CreateChangeSetRequest {
  changeSetName: string;
}

interface CreateChangeSetResponse {
  changeSet: ChangeSet;
  editSession: EditSession;
}

export function createChangeSet(
  request: CreateChangeSetRequest,
): Observable<ApiResponse<CreateChangeSetResponse>> {
  const bottle = Bottle.pop("default");
  const sdf: SDF = bottle.container.SDF;
  return sdf
    .post<ApiResponse<CreateChangeSetResponse>>(
      "change_set/create_change_set",
      request,
    )
    .pipe(
      tap((response) => {
        if (!response.error) {
          changeSet$.next(response.changeSet);
          editSession$.next(response.editSession);
          editMode$.next(true);
        }
      }),
    );
}