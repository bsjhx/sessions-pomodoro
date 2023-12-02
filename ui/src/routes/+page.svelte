<script>
    import Greet from '../lib/Greet.svelte'
</script>

<div>
    <button on:click="{greet}">Greet</button>
    <p>{greetMsg}</p>
</div>