<script>
    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from "svelte";
    import Calendar from "$lib/Calendar.svelte";

    let interval = 0;
    let timeDisplay = "";
    let currentState = {state_name: '', state_duration: 0};
    let timeout = 1000;
    let counterOverFlowed = false;
    let initialDuration = 0;

    let counter = 0;
    let additionalCounter = 0;

    let progress = 0;
    let additionalProgress = 0;

    let currentDate = new Date();

    let todayHistoryResponse = {
        states: []
    };

    onMount(async () => {
        initialDuration = await invoke('get_initial_time');
        todayHistoryResponse = await getTodayHistoryResponse();

        const c = await invoke('get_current_state');
        console.log('cs ', c);
        console.log('id ', initialDuration);

        currentState = {
            state_name: c.state_name,
            state_duration: c.state_duration
        };

        if (c.is_runnable) {
            interval = setInterval(onIntervalHandler, timeout);
            counter = c.time_left;
        } else {
            counter = initialDuration;
        }

        if (c.overtime > 0) {
            counterOverFlowed = true;
            additionalCounter = c.overtime;
            additionalProgress = (additionalCounter / (counter + additionalCounter)) * 100;
        }

        timeDisplay = updateClock(counter + additionalCounter);
    });

    async function getTodayHistoryResponse() {
        return await invoke('get_states_for_day', {day: currentDate});
    }

    async function startCycle() {
        currentState = await invoke('start_cycle');
        todayHistoryResponse = await getTodayHistoryResponse();
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
        todayHistoryResponse = await getTodayHistoryResponse();

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
        todayHistoryResponse = await getTodayHistoryResponse();
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
        let minutes = Math.floor(i / 60) % 60;
        let hours = Math.floor(i / 3600) % 3600;
        return "".concat(renderTimeNumber(hours), ':', renderTimeNumber(minutes), ':', renderTimeNumber(seconds));
    }

    function renderTimeNumber(n) {
        return n < 10 ? "".concat('0', n.toString()) : n.toString();
    }

    async function previousDay() {
        currentDate = new Date(currentDate.setDate(currentDate.getDate() - 1));
        todayHistoryResponse = await getTodayHistoryResponse();
    }

    async function nextDay() {
        currentDate = new Date(currentDate.setDate(currentDate.getDate() + 1));
        todayHistoryResponse = await getTodayHistoryResponse();
    }

    function displayDate(date) {
        let res = '';
        if (date.getDate() < 10) {
            res += '0' + date.getDate();
        } else {
            res += date.getDate();
        }
        res += '.';
        if (date.getMonth() < 9) {
            res += '0' + (date.getMonth() + 1);
        } else {
            res += (date.getMonth() + 1);
        }
        return res;
    }

</script>

<div class="container" style="margin-top: 30px">
    <div class="row">
        <div class="container text-center card col m-1">
            <div class="card-body">
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
        <div id="testId" class="text-center card m-1 col">
            <div>
                <div class="card-body mt-2">
                    <div style="margin-bottom: 20px">
                        <button type="button" class="btn btn-info" on:click='{previousDay}'>{'<'}</button>
                        {displayDate(currentDate)}
                        <button type="button" class="btn btn-info" on:click='{nextDay}'>{">"}</button>
                    </div>
                    <Calendar states={todayHistoryResponse.states}></Calendar>
                </div>
            </div>
        </div>
    </div>
</div>
