<script>
    import {invoke} from '@tauri-apps/api/tauri'

    let currentState = ["NothingState", "Biedronka"];

    async function start_cycle() {
        let new_state = await invoke('start_cycle');
        currentState = [...currentState, new_state];
    }
</script>

<div>
    <button on:click="{start_cycle}">Greet</button>
    <ol>
        {#each currentState as state}
            <li>{state}</li>
        {/each}
    </ol>
</div>