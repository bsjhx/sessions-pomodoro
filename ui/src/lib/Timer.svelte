<script>
    import {invoke} from '@tauri-apps/api/tauri'

    export let startTime = 0;

    let interval = 0;
    let timeDisplay = "00:00";
    let currentState = "NothingState";

    async function startCycle() {
        currentState = await invoke('start_cycle');
        if (interval === 0) {
            interval = setInterval(increment, 50);
        }
    }

    function stopCycle() {
        clearInterval(interval);
        interval = 0;
    }

    function displayTimer(i) {
        let seconds = i % 60;
        let minutes = Math.floor(i/60) % 3600;
        return  "".concat(renderTimeNumber(minutes), ':', renderTimeNumber(seconds));
    }

    function renderTimeNumber(n) {
        return n < 10 ? "".concat('0', n.toString()) : n.toString();
    }

    function increment() {
        startTime++;
        timeDisplay = displayTimer(startTime);
    }

</script>

<div>
    <p>{currentState}</p>
    <button disabled='{interval > 0}' on:click='{startCycle}'>Start cycle</button>
    <button disabled='{interval === 0}' on:click="{stopCycle}">Stop cycle</button>
    {timeDisplay}
</div>