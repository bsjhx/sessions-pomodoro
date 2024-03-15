<script>
    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from "svelte";
    import Calendar from "$lib/Calendar.svelte";

    let interval = 0;
    let timeDisplay = "";
    let currentState = {state_name: '', state_duration: 0};
    let timeout = 20;
    let counterOverFlowed = false;
    let initialDuration = 0;

    let counter = 0;
    let additionalCounter = 0;

    let progress = 0;
    let additionalProgress = 0;

    let todayHistoryResponse = {
        states: []
    };

    onMount(async () => {
        initialDuration =  await invoke('get_initial_time');
        todayHistoryResponse = await invoke('get_states_for_day', { day: new Date() });
        console.log(todayHistoryResponse);

        currentState = {
            state_name: 'NothingState',
            state_duration: initialDuration
        };

        counter = currentState.state_duration;
        timeDisplay = updateClock(counter);
    });

    async function startCycle() {
        currentState = await invoke('start_cycle');
        todayHistoryResponse = await invoke('get_states_for_day', { day: new Date() });
        if (interval === 0) {
            interval = setInterval(onIntervalHandler, timeout);
        }
    }

    function onIntervalHandler() {
        if (counterOverFlowed) {
            additionalCounter++;
            progress = (counter / (counter + additionalCounter)) * 100;
            additionalProgress = (additionalCounter / (counter + additionalCounter)) * 100;
        } else {
            counter--;
            progress = ((currentState.state_duration - counter) / currentState.state_duration) * 100;
            if (counter <= 0) {
                counterOverFlowed = true;
                counter = currentState.state_duration;
            }
        }

        timeDisplay = updateClock(counter + additionalCounter);
    }

    async function finishCycle() {
        currentState = await invoke('finish_cycle');
        todayHistoryResponse = await invoke('get_states_for_day', { day: new Date() });

        counter = initialDuration;
        timeDisplay = updateClock(counter);
        clearInterval(interval);
        interval = 0;
        progress = 0;
        additionalProgress = 0;
        additionalCounter = 0;

        counterOverFlowed = false;
    }

    async function endCurrentSession() {
        currentState = await invoke('end_current_session');
        todayHistoryResponse = await invoke('get_states_for_day', { day: new Date() });
        counter = currentState.state_duration;
        timeDisplay = updateClock(counter);
        clearInterval(interval);
        interval = setInterval(onIntervalHandler, timeout);

        progress = 0;
        additionalProgress = 0;
        additionalCounter = 0;

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

    function getTime(date) {
        return `${date.getHours()}:${date.getMinutes()}`
    }
</script>

<div class="container">
    <div class="row">
        <div class="container text-center card col m-1">
            <div class="card-body mt-2">
                <div class="row m-3">
                    <div class="col">{currentState.state_name}</div>
                </div>

                <div class="row m-5">
                    <div class="col"><h1><span class="badge text-bg-info">{timeDisplay}</span></h1></div>
                </div>

                <div class="mt-2">
                    <div class="progress-stacked">
                        <div class="progress" role="progressbar" aria-label="Segment one" aria-valuenow="15"
                             aria-valuemin="0" aria-valuemax="100" style="width: {progress}%">
                            <div class="progress-bar"></div>
                        </div>
                        <div class="progress" role="progressbar" aria-label="Segment two" aria-valuenow="30"
                             aria-valuemin="0" aria-valuemax="100" style="width: {additionalProgress}%">
                            <div class="progress-bar bg-success"></div>
                        </div>
                    </div>
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
        <div class="text-center card m-1 col">
            <div class="overflow-auto">
                <div class="card-body mt-2 anyClass">
                    <p>Today's statistics:</p>
                    <Calendar states={todayHistoryResponse.states}></Calendar>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .anyClass {
        height:80vh;
        overflow-y: scroll;
    }
</style>