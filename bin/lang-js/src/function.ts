export enum FunctionKind {
  QualificationCheck = "qualificationcheck",
  ResolverFunction = "resolverfunction",
  WorkflowResolve = "workflowResolve",
  CommandRun = "commandRun",
  Confirmation = "confirmation",
  Validation = "validation",
}

export function function_kinds(): Array<string> {
  return [
    FunctionKind.QualificationCheck,
    FunctionKind.ResolverFunction,
    FunctionKind.WorkflowResolve,
    FunctionKind.CommandRun,
    FunctionKind.Confirmation,
    FunctionKind.Validation,
  ];
}

export type Parameters = Record<string, unknown>;

export interface Request {
  executionId: string;
}

export interface RequestWithCode extends Request {
  handler: string;
  codeBase64: string;
}

export interface Result {
  protocol: "result";
}

export interface ResultSuccess extends Result {
  status: "success";
  executionId: string;
  error?: string;
}

export interface ResultFailure extends Result {
  status: "failure";
  executionId: string;
  error: {
    kind: string;
    message: string;
  };
}

export function failureExecution(
  err: Error,
  executionId: string
): ResultFailure {
  // `executionId` may not have been determined if the request JSON fails to
  // parse, message is malformed, etc. In this case an empty string can signal
  // that an id could not be determined at this point
  if (!executionId) {
    executionId = "";
  }
  return {
    protocol: "result",
    status: "failure",
    executionId,
    error: {
      kind: err.name,
      message: err.message,
    },
  };
}

export interface OutputLine {
  protocol: "output";
  executionId: string;
  stream: "stdout" | "stderr";
  level: "debug" | "info" | "warn" | "error";
  group?: string;
  message: string;
}
