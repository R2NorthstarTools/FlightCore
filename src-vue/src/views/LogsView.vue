<template>
    <el-dialog
        v-model="dialogVisible"
        :title="dialog_title"
        width="75%"
        :before-close="closeLog"
    >
        <el-input
            v-model="log_content"
            :autosize="{ minRows: 10, maxRows: 25 }"
            type="textarea"
            readonly
        />
    </el-dialog>

    <div class="fc-container" style="display: flex">
        <!-- Message displayed if no mod matched searched words -->
        <div v-if="logs.length === 0" class="noModMessage">
            {{ $t('logs.no_logs') }}
        </div>

        <el-scrollbar v-else>
            <el-card v-for="log of logs" shadow="hover">
               <el-button type="success" @click="openLog(log)">
                    {{ $t('logs.open') }}
                </el-button>

                {{log.filename}}
            </el-card>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";

const dialogVisible = ref(true);

export default defineComponent({
    name: "LogsView",
    data: () => ({
        dialogVisible: false,
        dialog_title: null
    }),
    computed: {
        logs(): NorthstarLog[] {
            return this.$store.state.northstar_logs || [];
        },
        log_content(): string {
            return this.$store.state.northstar_log_content || "Loading"
        }
    },
    methods: {
        openLog(log: NorthstarLog) {
            this.$store.commit('loadNorthstarLog', log);

            this.dialog_title = log.filename;
            this.dialogVisible = true;
        },

        closeLog(done: () => void)
        {
            this.dialog_title = null
            done();
        }
    },
    mounted() {
        // Fetch Thunderstore logs
        this.$store.commit('fetchNorthstarLogs');
    }
});
</script>

<style scoped>
.fc_mods__container {
    display: flex;
    width: 100%;
    flex-direction: column;
}

</style>
