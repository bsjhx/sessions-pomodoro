<script>
    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from "svelte";

    let counter = 0;
    let interval = 0;
    let timeDisplay = "";
    let currentState = "";
    let timeout = 500;
    let counterOverFlowed = false;

    let times;

    onMount(async () => {
        currentState = "NothingState";

        times = await invoke('get_times');
        counter = times.workingTime;
        timeDisplay = updateClock(counter);

    });

    async function startCycle() {
        currentState = await invoke('start_cycle');
        if (interval === 0) {
            interval = setInterval(onIntervalHandler, timeout);
        }
    }

    function onIntervalHandler() {
        if (counter <= 0) {
            counterOverFlowed = true;
            counter = times[currentState];
        }

        if (counterOverFlowed) {
            counter++;
        } else {
            counter--;
        }

        timeDisplay = updateClock(counter);
    }

    async function finishCycle() {
        currentState = await invoke('finish_cycle');

        counter = times['WorkingTimeState'];
        timeDisplay = updateClock(counter);
        clearInterval(interval);
        interval = 0;
        counterOverFlowed = false;
    }

    async function endCurrentSession() {
        currentState = await invoke('end_current_session');
        counter = times[currentState];
        timeDisplay = updateClock(counter);
        clearInterval(interval);
        interval = setInterval(onIntervalHandler, timeout);

        counterOverFlowed = false;
    }

    function updateClock(i) {
        let seconds = i % 60;
        let minutes = Math.floor(i / 60) % 3600;
        return "".concat(renderTimeNumber(minutes), ':', renderTimeNumber(seconds));
    }

    function renderTimeNumber(n) {
        return n < 10 ? "".concat('0', n.toString()) : n.toString();
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
            {#if currentState === 'NothingState'}
                <div class="col">
                    <button type="button" class="btn btn-primary" disabled='{interval > 0}' on:click='{startCycle}'>
                        Start
                        cycle
                    </button>
                </div>
            {/if}
            {#if currentState !== 'NothingState'}
                <div class="col">
                    <button type="button" class="btn btn-danger" disabled='{interval === 0}' on:click="{finishCycle}">
                        Stop
                        cycle
                    </button>
                </div>
                <div class="col">
                    <button type="button" class="btn btn-secondary" disabled='{currentState === "NothigState"}'
                            on:click='{endCurrentSession}'>End
                    </button>
                </div>
            {/if}
        </div>
    </div>
</div>