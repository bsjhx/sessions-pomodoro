<script>
    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from "svelte";

    let counter = 0;
    let interval = 0;
    let timeDisplay = "";
    let currentState = {state_name: '', state_duration: 0};
    let timeout = 500;
    let counterOverFlowed = false;
    let initialDuration = 0;

    onMount(async () => {
        initialDuration = await invoke('get_initial_time');

        currentState = {
            state_name: 'NothingState',
            state_duration: initialDuration
        };

        counter = currentState.state_duration;
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
            counter = currentState.state_duration;
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

        counter = initialDuration;
        timeDisplay = updateClock(counter);
        clearInterval(interval);
        interval = 0;
        counterOverFlowed = false;
    }

    async function endCurrentSession() {
        currentState = await invoke('end_current_session');
        counter = currentState.state_duration;
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
            <div class="col">{currentState.state_name}</div>
        </div>

        <div class="row m-5">
            <div class="col"><h1><span class="badge text-bg-info">{timeDisplay}</span></h1></div>
        </div>

        <div class="row m-5">
            {#if currentState.state_name === 'NothingState'}
                <div class="col">
                    <button type="button" class="btn btn-primary" on:click='{startCycle}'>
                        Start
                        cycle
                    </button>
                </div>
            {/if}
            {#if currentState.state_name !== 'NothingState'}
                <div class="col">
                    <button type="button" class="btn btn-danger" on:click="{finishCycle}">
                        Stop
                        cycle
                    </button>
                </div>
                <div class="col">
                    <button type="button" class="btn btn-secondary"
                            on:click='{endCurrentSession}'>End
                    </button>
                </div>
            {/if}
        </div>
    </div>
</div>