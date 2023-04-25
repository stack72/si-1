import {posthog} from "./posthog";

export function isEnabled(
    featureFlag: string,
) {
    return posthog.isFeatureEnabled(featureFlag)
}
