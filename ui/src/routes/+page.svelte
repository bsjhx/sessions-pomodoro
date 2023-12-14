<script>
    import {invoke} from '@tauri-apps/api/tauri'

    export let currentState = "NothingState";
    export let interval = 0;
    export let timeDisplay = "00:00";

    async function startCycle() {
        currentState = await invoke('start_cycle');
        interval = setInterval(increment, 50);
    }

    async function stopCycle() {
        // currentState = await invoke('start_cycle');
        clearInterval(interval);
    }

    function displayTimer(i) {
        let seconds = i % 60;
        let minutes = Math.floor(i/60) % 3600;
        return  "".concat(renderTimeNumber(minutes), ':', renderTimeNumber(seconds));
    }

    function renderTimeNumber(n) {
        return n < 10 ? "".concat('0', n.toString()) : n.toString();
    }

    export let i = 0

    function increment() {
        i++;
        timeDisplay = displayTimer(i);
    }

</script>

<div>
    <h3>Pomodoro</h3>
    <button on:click="{startCycle}">Start cycle</button>
    <button on:click="{stopCycle}">Stop cycle</button>
    {timeDisplay}
</div>