<script>
    import { invoke } from '@tauri-apps/api/tauri'

    async function start_cycle() {
        await invoke('start_cycle')
    }
</script>

<div>
    <button on:click="{start_cycle}">Greet</button>
</div>