"use strict";

var _registry = require("../../registry");

_registry.registry.componentAndEntity({
  typeName: "kubernetesDeployment",
  displayTypeName: "Kubernetes Deployment Object",
  siPathName: "si-kubernetes",
  serviceName: "kubernetes",
  options: function options(c) {
    c.entity.associations.belongsTo({
      fromFieldPath: ["siProperties", "billingAccountId"],
      typeName: "billingAccount"
    });
    c.entity.integrationServices.push({
      integrationName: "aws",
      integrationServiceName: "eks_kubernetes"
    }); // Constraints

    c.constraints.addEnum({
      name: "kubernetesVersion",
      label: "Kubernetes Version",
      options: function options(p) {
        p.variants = ["v1.12", "v1.13", "v1.14", "v1.15"];
      }
    }); // Properties

    c.properties.addObject({
      name: "kubernetesObject",
      label: "Kubernetes Object",
      options: function options(p) {
        p.relationships.updates({
          partner: {
            typeName: "kubernetesDeploymentEntity",
            names: ["properties", "kubernetesObjectYaml"]
          }
        });
        p.relationships.either({
          partner: {
            typeName: "kubernetesDeploymentEntity",
            names: ["properties", "kubernetesObjectYaml"]
          }
        });
        p.properties.addText({
          name: "apiVersion",
          label: "API Version",
          options: function options(p) {
            p.required = true;
          }
        });
        p.properties.addText({
          name: "kind",
          label: "Kind",
          options: function options(p) {
            p.required = true;
            p.baseDefaultValue = "Deployment";
          }
        });
        p.properties.addLink({
          name: "metadata",
          label: "Metadata",
          options: function options(p) {
            p.lookup = {
              typeName: "kubernetesMetadata"
            };
          }
        });
        p.properties.addObject({
          name: "spec",
          label: "Deployment Spec",
          options: function options(p) {
            p.properties.addNumber({
              name: "replicas",
              label: "Replicas",
              options: function options(p) {
                p.numberKind = "int32";
              }
            });
            p.properties.addLink({
              name: "selector",
              label: "Selector",
              options: function options(p) {
                p.lookup = {
                  typeName: "kubernetesSelector"
                };
              }
            });
            p.properties.addLink({
              name: "template",
              label: "Pod Template Spec",
              options: function options(p) {
                p.lookup = {
                  typeName: "kubernetesPodTemplateSpec"
                };
              }
            });
          }
        });
      }
    });
    c.properties.addCode({
      name: "kubernetesObjectYaml",
      label: "Kubernetes Object YAML",
      options: function options(p) {
        p.relationships.updates({
          partner: {
            typeName: "kubernetesDeploymentEntity",
            names: ["properties", "kubernetesObject"]
          }
        });
        p.relationships.either({
          partner: {
            typeName: "kubernetesDeploymentEntity",
            names: ["properties", "kubernetesObject"]
          }
        });
        p.language = "yaml";
      }
    }); // Entity Actions

    c.entity.methods.addAction({
      name: "apply",
      label: "Apply",
      options: function options(p) {
        p.mutation = true;
      }
    });
  }
});
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uL3NyYy9jb21wb25lbnRzL3NpLWt1YmVybmV0ZXMvZGVwbG95bWVudC50cyJdLCJuYW1lcyI6WyJyZWdpc3RyeSIsImNvbXBvbmVudEFuZEVudGl0eSIsInR5cGVOYW1lIiwiZGlzcGxheVR5cGVOYW1lIiwic2lQYXRoTmFtZSIsInNlcnZpY2VOYW1lIiwib3B0aW9ucyIsImMiLCJlbnRpdHkiLCJhc3NvY2lhdGlvbnMiLCJiZWxvbmdzVG8iLCJmcm9tRmllbGRQYXRoIiwiaW50ZWdyYXRpb25TZXJ2aWNlcyIsInB1c2giLCJpbnRlZ3JhdGlvbk5hbWUiLCJpbnRlZ3JhdGlvblNlcnZpY2VOYW1lIiwiY29uc3RyYWludHMiLCJhZGRFbnVtIiwibmFtZSIsImxhYmVsIiwicCIsInZhcmlhbnRzIiwicHJvcGVydGllcyIsImFkZE9iamVjdCIsInJlbGF0aW9uc2hpcHMiLCJ1cGRhdGVzIiwicGFydG5lciIsIm5hbWVzIiwiZWl0aGVyIiwiYWRkVGV4dCIsInJlcXVpcmVkIiwiYmFzZURlZmF1bHRWYWx1ZSIsImFkZExpbmsiLCJsb29rdXAiLCJhZGROdW1iZXIiLCJudW1iZXJLaW5kIiwiYWRkQ29kZSIsImxhbmd1YWdlIiwibWV0aG9kcyIsImFkZEFjdGlvbiIsIm11dGF0aW9uIl0sIm1hcHBpbmdzIjoiOztBQVNBOztBQUVBQSxtQkFBU0Msa0JBQVQsQ0FBNEI7QUFDMUJDLEVBQUFBLFFBQVEsRUFBRSxzQkFEZ0I7QUFFMUJDLEVBQUFBLGVBQWUsRUFBRSw4QkFGUztBQUcxQkMsRUFBQUEsVUFBVSxFQUFFLGVBSGM7QUFJMUJDLEVBQUFBLFdBQVcsRUFBRSxZQUphO0FBSzFCQyxFQUFBQSxPQUwwQixtQkFLbEJDLENBTGtCLEVBS2Y7QUFDVEEsSUFBQUEsQ0FBQyxDQUFDQyxNQUFGLENBQVNDLFlBQVQsQ0FBc0JDLFNBQXRCLENBQWdDO0FBQzlCQyxNQUFBQSxhQUFhLEVBQUUsQ0FBQyxjQUFELEVBQWlCLGtCQUFqQixDQURlO0FBRTlCVCxN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LE9BQU8sRUFBRTtBQUNQeEIsWUFBQUEsUUFBUSxFQUFFLDRCQURIO0FBRVB5QixZQUFBQSxLQUFLLEVBQUUsQ0FBQyxZQUFELEVBQWUsa0JBQWY7QUFGQTtBQURZLFNBQXZCO0FBTUFQLFFBQUFBLENBQUMsQ0FBQ2lCLFFBQUYsR0FBYSxNQUFiO0FBQ0Q7QUFqQmtCLEtBQXJCLEVBN0ZTLENBaUhUOztBQUNBOUIsSUFBQUEsQ0FBQyxDQUFDQyxNQUFGLENBQVM4QixPQUFULENBQWlCQyxTQUFqQixDQUEyQjtBQUN6QnJCLE1BQUFBLElBQUksRUFBRSxPQURtQjtBQUV6QkMsTUFBQUEsS0FBSyxFQUFFLE9BRmtCO0FBR3pCYixNQUFBQSxPQUh5QixtQkFHakJjLENBSGlCLEVBR0Y7QUFDckJBLFFBQUFBLENBQUMsQ0FBQ29CLFFBQUYsR0FBYSxJQUFiO0FBQ0Q7QUFMd0IsS0FBM0I7QUFPRDtBQTlIeUIsQ0FBNUIiLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQge1xuICBQcm9wT2JqZWN0LFxuICBQcm9wVGV4dCxcbiAgUHJvcExpbmssXG4gIFByb3BOdW1iZXIsXG4gIFByb3BFbnVtLFxuICBQcm9wQ29kZSxcbiAgUHJvcEFjdGlvbixcbn0gZnJvbSBcIi4uLy4uL2NvbXBvbmVudHMvcHJlbHVkZVwiO1xuaW1wb3J0IHsgcmVnaXN0cnkgfSBmcm9tIFwiLi4vLi4vcmVnaXN0cnlcIjtcblxucmVnaXN0cnkuY29tcG9uZW50QW5kRW50aXR5KHtcbiAgdHlwZU5hbWU6IFwia3ViZXJuZXRlc0RlcGxveW1lbnRcIixcbiAgZGlzcGxheVR5cGVOYW1lOiBcIkt1YmVybmV0ZXMgRGVwbG95bWVudCBPYmplY3RcIixcbiAgc2lQYXRoTmFtZTogXCJzaS1rdWJlcm5ldGVzXCIsXG4gIHNlcnZpY2VOYW1lOiBcImt1YmVybmV0ZXNcIixcbiAgb3B0aW9ucyhjKSB7XG4gICAgYy5lbnRpdHkuYXNzb2NpYXRpb25zLmJlbG9uZ3NUbyh7XG4gICAgICBmcm9tRmllbGRQYXRoOiBbXCJzaVByb3BlcnRpZXNcIiwgXCJiaWxsaW5nQWNjb3VudElkXCJdLFxuICAgICAgdHlwZU5hbWU6IFwiYmlsbGluZ0FjY291bnRcIixcbiAgICB9KTtcbiAgICBjLmVudGl0eS5pbnRlZ3JhdGlvblNlcnZpY2VzLnB1c2goe1xuICAgICAgaW50ZWdyYXRpb25OYW1lOiBcImF3c1wiLFxuICAgICAgaW50ZWdyYXRpb25TZXJ2aWNlTmFtZTogXCJla3Nfa3ViZXJuZXRlc1wiLFxuICAgIH0pO1xuXG4gICAgLy8gQ29uc3RyYWludHNcbiAgICBjLmNvbnN0cmFpbnRzLmFkZEVudW0oe1xuICAgICAgbmFtZTogXCJrdWJlcm5ldGVzVmVyc2lvblwiLFxuICAgICAgbGFiZWw6IFwiS3ViZXJuZXRlcyBWZXJzaW9uXCIsXG4gICAgICBvcHRpb25zKHA6IFByb3BFbnVtKSB7XG4gICAgICAgIHAudmFyaWFudHMgPSBbXCJ2MS4xMlwiLCBcInYxLjEzXCIsIFwidjEuMTRcIiwgXCJ2MS4xNVwiXTtcbiAgICAgIH0sXG4gICAgfSk7XG5cbiAgICAvLyBQcm9wZXJ0aWVzXG4gICAgYy5wcm9wZXJ0aWVzLmFkZE9iamVjdCh7XG4gICAgICBuYW1lOiBcImt1YmVybmV0ZXNPYmplY3RcIixcbiAgICAgIGxhYmVsOiBcIkt1YmVybmV0ZXMgT2JqZWN0XCIsXG4gICAgICBvcHRpb25zKHA6IFByb3BPYmplY3QpIHtcbiAgICAgICAgcC5yZWxhdGlvbnNoaXBzLnVwZGF0ZXMoe1xuICAgICAgICAgIHBhcnRuZXI6IHtcbiAgICAgICAgICAgIHR5cGVOYW1lOiBcImt1YmVybmV0ZXNEZXBsb3ltZW50RW50aXR5XCIsXG4gICAgICAgICAgICBuYW1lczogW1wicHJvcGVydGllc1wiLCBcImt1YmVybmV0ZXNPYmplY3RZYW1sXCJdLFxuICAgICAgICAgIH0sXG4gICAgICAgIH0pO1xuICAgICAgICBwLnJlbGF0aW9uc2hpcHMuZWl0aGVyKHtcbiAgICAgICAgICBwYXJ0bmVyOiB7XG4gICAgICAgICAgICB0eXBlTmFtZTogXCJrdWJlcm5ldGVzRGVwbG95bWVudEVudGl0eVwiLFxuICAgICAgICAgICAgbmFtZXM6IFtcInByb3BlcnRpZXNcIiwgXCJrdWJlcm5ldGVzT2JqZWN0WWFtbFwiXSxcbiAgICAgICAgICB9LFxuICAgICAgICB9KTtcbiAgICAgICAgcC5wcm9wZXJ0aWVzLmFkZFRleHQoe1xuICAgICAgICAgIG5hbWU6IFwiYXBpVmVyc2lvblwiLFxuICAgICAgICAgIGxhYmVsOiBcIkFQSSBWZXJzaW9uXCIsXG4gICAgICAgICAgb3B0aW9ucyhwOiBQcm9wVGV4dCkge1xuICAgICAgICAgICAgcC5yZXF1aXJlZCA9IHRydWU7XG4gICAgICAgICAgfSxcbiAgICAgICAgfSk7XG4gICAgICAgIHAucHJvcGVydGllcy5hZGRUZXh0KHtcbiAgICAgICAgICBuYW1lOiBcImtpbmRcIixcbiAgICAgICAgICBsYWJlbDogXCJLaW5kXCIsXG4gICAgICAgICAgb3B0aW9ucyhwOiBQcm9wVGV4dCkge1xuICAgICAgICAgICAgcC5yZXF1aXJlZCA9IHRydWU7XG4gICAgICAgICAgICBwLmJhc2VEZWZhdWx0VmFsdWUgPSBcIkRlcGxveW1lbnRcIjtcbiAgICAgICAgICB9LFxuICAgICAgICB9KTtcbiAgICAgICAgcC5wcm9wZXJ0aWVzLmFkZExpbmsoe1xuICAgICAgICAgIG5hbWU6IFwibWV0YWRhdGFcIixcbiAgICAgICAgICBsYWJlbDogXCJNZXRhZGF0YVwiLFxuICAgICAgICAgIG9wdGlvbnMocDogUHJvcExpbmspIHtcbiAgICAgICAgICAgIHAubG9va3VwID0ge1xuICAgICAgICAgICAgICB0eXBlTmFtZTogXCJrdWJlcm5ldGVzTWV0YWRhdGFcIixcbiAgICAgICAgICAgIH07XG4gICAgICAgICAgfSxcbiAgICAgICAgfSk7XG4gICAgICAgIHAucHJvcGVydGllcy5hZGRPYmplY3Qoe1xuICAgICAgICAgIG5hbWU6IFwic3BlY1wiLFxuICAgICAgICAgIGxhYmVsOiBcIkRlcGxveW1lbnQgU3BlY1wiLFxuICAgICAgICAgIG9wdGlvbnMocDogUHJvcE9iamVjdCkge1xuICAgICAgICAgICAgcC5wcm9wZXJ0aWVzLmFkZE51bWJlcih7XG4gICAgICAgICAgICAgIG5hbWU6IFwicmVwbGljYXNcIixcbiAgICAgICAgICAgICAgbGFiZWw6IFwiUmVwbGljYXNcIixcbiAgICAgICAgICAgICAgb3B0aW9ucyhwOiBQcm9wTnVtYmVyKSB7XG4gICAgICAgICAgICAgICAgcC5udW1iZXJLaW5kID0gXCJpbnQzMlwiO1xuICAgICAgICAgICAgICB9LFxuICAgICAgICAgICAgfSk7XG4gICAgICAgICAgICBwLnByb3BlcnRpZXMuYWRkTGluayh7XG4gICAgICAgICAgICAgIG5hbWU6IFwic2VsZWN0b3JcIixcbiAgICAgICAgICAgICAgbGFiZWw6IFwiU2VsZWN0b3JcIixcbiAgICAgICAgICAgICAgb3B0aW9ucyhwOiBQcm9wTGluaykge1xuICAgICAgICAgICAgICAgIHAubG9va3VwID0ge1xuICAgICAgICAgICAgICAgICAgdHlwZU5hbWU6IFwia3ViZXJuZXRlc1NlbGVjdG9yXCIsXG4gICAgICAgICAgICAgICAgfTtcbiAgICAgICAgICAgICAgfSxcbiAgICAgICAgICAgIH0pO1xuICAgICAgICAgICAgcC5wcm9wZXJ0aWVzLmFkZExpbmsoe1xuICAgICAgICAgICAgICBuYW1lOiBcInRlbXBsYXRlXCIsXG4gICAgICAgICAgICAgIGxhYmVsOiBcIlBvZCBUZW1wbGF0ZSBTcGVjXCIsXG4gICAgICAgICAgICAgIG9wdGlvbnMocDogUHJvcExpbmspIHtcbiAgICAgICAgICAgICAgICBwLmxvb2t1cCA9IHtcbiAgICAgICAgICAgICAgICAgIHR5cGVOYW1lOiBcImt1YmVybmV0ZXNQb2RUZW1wbGF0ZVNwZWNcIixcbiAgICAgICAgICAgICAgICB9O1xuICAgICAgICAgICAgICB9LFxuICAgICAgICAgICAgfSk7XG4gICAgICAgICAgfSxcbiAgICAgICAgfSk7XG4gICAgICB9LFxuICAgIH0pO1xuICAgIGMucHJvcGVydGllcy5hZGRDb2RlKHtcbiAgICAgIG5hbWU6IFwia3ViZXJuZXRlc09iamVjdFlhbWxcIixcbiAgICAgIGxhYmVsOiBcIkt1YmVybmV0ZXMgT2JqZWN0IFlBTUxcIixcbiAgICAgIG9wdGlvbnMocDogUHJvcENvZGUpIHtcbiAgICAgICAgcC5yZWxhdGlvbnNoaXBzLnVwZGF0ZXMoe1xuICAgICAgICAgIHBhcnRuZXI6IHtcbiAgICAgICAgICAgIHR5cGVOYW1lOiBcImt1YmVybmV0ZXNEZXBsb3ltZW50RW50aXR5XCIsXG4gICAgICAgICAgICBuYW1lczogW1wicHJvcGVydGllc1wiLCBcImt1YmVybmV0ZXNPYmplY3RcIl0sXG4gICAgICAgICAgfSxcbiAgICAgICAgfSk7XG4gICAgICAgIHAucmVsYXRpb25zaGlwcy5laXRoZXIoe1xuICAgICAgICAgIHBhcnRuZXI6IHtcbiAgICAgICAgICAgIHR5cGVOYW1lOiBcImt1YmVybmV0ZXNEZXBsb3ltZW50RW50aXR5XCIsXG4gICAgICAgICAgICBuYW1lczogW1wicHJvcGVydGllc1wiLCBcImt1YmVybmV0ZXNPYmplY3RcIl0sXG4gICAgICAgICAgfSxcbiAgICAgICAgfSk7XG4gICAgICAgIHAubGFuZ3VhZ2UgPSBcInlhbWxcIjtcbiAgICAgIH0sXG4gICAgfSk7XG5cbiAgICAvLyBFbnRpdHkgQWN0aW9uc1xuICAgIGMuZW50aXR5Lm1ldGhvZHMuYWRkQWN0aW9uKHtcbiAgICAgIG5hbWU6IFwiYXBwbHlcIixcbiAgICAgIGxhYmVsOiBcIkFwcGx5XCIsXG4gICAgICBvcHRpb25zKHA6IFByb3BBY3Rpb24pIHtcbiAgICAgICAgcC5tdXRhdGlvbiA9IHRydWU7XG4gICAgICB9LFxuICAgIH0pO1xuICB9LFxufSk7XG4iXX0=