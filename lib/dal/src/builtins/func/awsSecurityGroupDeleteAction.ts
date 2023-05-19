async function deleteResource(component: Input): Promise<Output> {
  const resource = component.properties.resource?.payload;
  // Now, delete the security group.
  const child = await siExec.waitUntilEnd("aws", [
    "ec2",
    "delete-security-group",
    "--region",
    component.properties.domain.region,
    "--group-id",
    resource.GroupId,
  ]);

  if (child.exitCode !== 0) {
    console.error(child.stderr);
    if (child.stderr.includes("DependencyViolation")) {
      return {
        status: "error",
        payload: resource,
        message: `Unable to delete Security Group while it is in use: ${child.exitCode}`,
      };
    } else {
      return {
        status: "error",
        payload: resource,
        message: `Unable to delete Security Group, AWS CLI 2 exited with non zero code: ${child.exitCode}`,
      };
    }
  }

  return { payload: null, status: "ok" };
}