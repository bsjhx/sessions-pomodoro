<script>
    import {invoke} from '@tauri-apps/api/tauri'
    import Timer from "$lib/Timer.svelte";

    let currentState = "NothingState";

    async function startCycle() {
        currentState = await invoke('start_cycle');
    }

    function stopCycle() {
        // currentState = await invoke('start_cycle');

    }

</script>

<div>
    <h3>Pomodoro</h3>
    <Timer></Timer>
</div>