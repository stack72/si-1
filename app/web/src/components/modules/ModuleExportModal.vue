<template>
  <Modal ref="modalRef" :title="title">
    <Stack>
      <VormInput
        v-model="packageExportReq.name"
        label="Name"
        required
        placeholder="The name of this module..."
      />
      <VormInput
        v-if="!autoVersion"
        v-model="packageExportReq.version"
        label="Version"
        required
        placeholder="The version of this module..."
      />
      <VormInput
        v-model="packageExportReq.description"
        label="Description"
        type="textarea"
        placeholder="Give this module a short description..."
      />
      <!--
      <div class="flex flex-row items-end gap-sm">
        <VormInput
          v-model="selectedSchemaVariant"
          label="Assets"
          type="dropdown"
          class="flex-1"
          :options="schemaVariantOptions"
        />
        <VButton
          label="Add"
          tone="action"
          icon="plus"
          size="xs"
          class="mb-1"
          @click="addSchemaVariantToExport"
        />
      </div>
      <ul>
        <li
          v-for="svId in schemaVariantsForExport"
          :key="svId"
          class="flex flex-row gap-sm px-1"
        >
          <div class="pr-2" role="decoration">•</div>
          {{ schemaVariantsById?.[svId]?.schemaName }}
          <div class="ml-auto">
            <VButton label="" icon="trash" @click="removeSchemaVariant(svId)" />
          </div>
        </li>
      </ul>
      -->
      <VueMultiselect
        v-model="selectedSchemaVariants"
        multiple
        :options="schemaVariantOptions"
        :customLabel="(opt: Option) => opt.label"
      />
      <ErrorMessage
        v-if="exportPkgReqStatus.isError"
        :requestStatus="exportPkgReqStatus"
      />
      <p>
        Assets contributed to System Initiative will be reviewed for sharing
        with the community.
      </p>
      <VButton
        :requestStatus="exportPkgReqStatus"
        loadingText="Exporting..."
        :disabled="!enableExportButton"
        label="Export"
        tone="action"
        icon="cloud-upload"
        size="sm"
        @click="exportPkg"
      />
    </Stack>
  </Modal>
</template>

<script lang="ts" setup>
import { ref, computed } from "vue";
import {
  Modal,
  VButton,
  VormInput,
  useModal,
  Stack,
  ErrorMessage,
} from "@si/vue-lib/design-system";
import { format as dateFormat } from "date-fns";
import VueMultiselect from "vue-multiselect";
import { useComponentsStore } from "@/store/components.store";
import { useModuleStore, PkgExportRequest } from "@/store/module.store";
import { Option } from "../SelectMenu.vue";

const moduleStore = useModuleStore();
const componentStore = useComponentsStore();
const modalRef = ref<InstanceType<typeof Modal>>();
const exportPkgReqStatus = moduleStore.getRequestStatus("EXPORT_MODULE");
const selectedSchemaVariants = ref<Option[]>([]);

const props = withDefaults(
  defineProps<{
    title?: string;
    label?: string;
    autoVersion?: boolean;
    preSelectedSchemaVariantId?: string;
  }>(),
  {
    title: "Export Module",
    label: "Export",
    autoVersion: false,
  },
);

const emptyExportPackageReq: PkgExportRequest = {
  name: "",
  description: undefined,
  version: "",
  schemaVariants: [],
};

const schemaVariantsForExport = ref<string[]>([]);

const packageExportReq = ref<PkgExportRequest>({ ...emptyExportPackageReq });

const { open: openModal, close } = useModal(modalRef);
const open = () => {
  selectedSchemaVariants.value = [];
  schemaVariantsForExport.value = [];
  if (props.preSelectedSchemaVariantId) {
    const preselectedOption = schemaVariantOptions.value.find(
      (opt) => opt.value === props.preSelectedSchemaVariantId,
    );
    if (preselectedOption) {
      selectedSchemaVariants.value = [preselectedOption];
    }
  }
  packageExportReq.value = { ...emptyExportPackageReq };
  openModal();
};

defineExpose({ open, close });

const schemaVariantOptions = computed(() =>
  componentStore.schemaVariants.map((sv) => ({
    label: sv.schemaName,
    value: sv.id,
  })),
);

const getVersionTimestamp = () => dateFormat(Date.now(), "yyyyMMddkkmmss");

const enableExportButton = computed(() => {
  if (packageExportReq.value?.name?.trim().length === 0) {
    return false;
  }
  if (
    !props.autoVersion &&
    packageExportReq.value?.version?.trim().length === 0
  ) {
    return false;
  }
  if (selectedSchemaVariants.value.length === 0) {
    return false;
  }

  return true;
});

const exportPkg = async () => {
  if (props.autoVersion) {
    packageExportReq.value.version = getVersionTimestamp();
  }
  const result = await moduleStore.EXPORT_MODULE({
    ...packageExportReq.value,
    schemaVariants: selectedSchemaVariants.value.map(
      (opt) => opt.value as string,
    ),
  });
  if (result.result.success) {
    close();
    await moduleStore.LOAD_LOCAL_MODULES();
  }
};
</script>

<style src="vue-multiselect/dist/vue-multiselect.css"></style>
