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

    async function finishCycle() {
        currentState = await invoke('finish_cycle');
        clearInterval(interval);
        interval = 0;
    }

    function displayTimer(i) {
        let seconds = i % 60;
        let minutes = Math.floor(i / 60) % 3600;
        return "".concat(renderTimeNumber(minutes), ':', renderTimeNumber(seconds));
    }

    function renderTimeNumber(n) {
        return n < 10 ? "".concat('0', n.toString()) : n.toString();
    }

    function increment() {
        startTime++;
        timeDisplay = displayTimer(startTime);
    }
</script>

<div class="container text-center card mt-4">
    <div class="card-body mt-2">
        <div class="row m-3">
            <div class="col">{currentState}</div>
        </div>

        <div class="row m-5">
            <div class="col"><h1><span class="badge text-bg-info">{timeDisplay}</span></h1></div>
        </div>

        <div class="row m-5">
            <div class="col">
            </div>
            <div class="col">
                <button type="button" class="btn btn-primary" disabled='{interval > 0}' on:click='{startCycle}'>Start
                    cycle
                </button>
            </div>
            <div class="col">
                <button type="button" class="btn btn-danger" disabled='{interval === 0}' on:click="{finishCycle}">Stop
                    cycle
                </button>
            </div>
            <div class="col">
            </div>
        </div>

    </div>
</div>