<script>
    import {onMount} from "svelte";
    import { afterUpdate } from 'svelte';

    const HOURS = ['00:00', '01:00', '02:00', '03:00', '04:00', '05:00', '06:00', '07:00', '08:00', '09:00', '10:00', '11:00', '12:00', '13:00', '14:00', '15:00', '16:00', '17:00', '18:00', '19:00', '20:00', '21:00', '22:00', '23:00'];

    export let states = [];

    let nowMarkerPosition = '0px';

    onMount(() => {
        let now = new Date();
        nowMarkerPosition = `${now.getHours() * 60 + now.getMinutes()}px`
        setInterval(onIntervalHandler, 10);
        const el = document.getElementById('state-1');
        console.log('elesss', el);
    });

    afterUpdate(() => {
        console.log('DOM changed');
        let el;
        if (states.length > 0 && new Date(states[0].started_time).getDate() === new Date().getDate() || states.length === 0) {
            el = document.getElementById('now-marker');
        } else {
            el = document.getElementById('state-0');
        }

        if (!el) return;
        el.scrollIntoView({
            behavior: 'smooth'
        });
    });

    function onIntervalHandler() {
        let now = new Date();
        nowMarkerPosition = `${now.getHours() * 60 + now.getMinutes()}px`
    }

    function getTime(date) {
        return `${date.getHours()}:${date.getMinutes()}`
    }

    // In calendar view 1 minute = 1 px
    function calculateGridRowFromDate(date) {
        return date.getHours() * 60 + date.getMinutes();
    }

    function getColor(state) {
        if (state.state_id === 'WorkingTimeState') {
            return '#a4f9c8';
        } else {
            return '#8c847d';
        }
    }

</script>

<div class="scrollable">
    <div class="dayview-container">
        <div class="dayview-timestrings-container">
            <div class="dayview-timestrings">
                {#each HOURS as hour}
                    <div class="dayview-timestring-container">
                        <div class="dayview-timestring">
                            {hour}
                        </div>
                    </div>
                {/each}
            </div>
        </div>
        <div class="dayview-grid-container">
            <div class="dayview-grid">
                <div class="dayview-grid-tiles">
                    {#each {length: HOURS.length} as _}
                        <div class="dayview-grid-tile"></div>
                    {/each}
                </div>
                <div class="dayview-now-marker" id="now-marker" style:top="{nowMarkerPosition}"></div>
                <div class="dayview-grid-marker-start"></div>
                <div class="dayview-gridcell-container">
                    <div class="dayview-gridcell">
                        {#each Object.entries(states) as [i, state]}
                            <div    id={`state-${i}`}
                                    class="dayview-cell dayview-cell-extended"
                                    style="background-color: {getColor(state)}; grid-row: {calculateGridRowFromDate(new Date(state.started_time))} / {calculateGridRowFromDate(new Date(state.finished_time))};"
                            >
                                {#if state.length_in_seconds > 500}
                                    <div class="dayview-cell-title">{state.state_id}</div>
                                {/if}
                                {#if state.length_in_seconds > 800}
                                    <div class="dayview-cell-time">{getTime(new Date(state.started_time))} - {getTime(new Date(state.finished_time))}</div>
                                {/if}
                            </div>
                        {/each}
                    </div>
                </div>
                <div class="dayview-grid-marker-end"></div>
            </div>
        </div>
    </div>
</div>

<style>
    .dayview-container {
        position: relative;
        display: flex;
        overflow: hidden;
        align-items: stretch;
        flex: 1 1 auto;
    }

    .dayview-timestrings-container {
        height: auto;
        overflow-y: hidden;
        flex: none;
        display: flex;
        align-items: flex-start;
        min-width: 40px;
    }

    .dayview-timestring-container {
        height: 60px;
        position: relative;
        padding-inline-end: 8px;
        text-align: right;
    }

    .dayview-timestring-container:first-child .dayview-timestring {
        display: none;
    }

    .dayview-timestring {
        display: block;
        color: #70757a;
        font-size: 10px;
        position: relative;
        top: -6px;
    }

    .dayview-timestrings {
        position: relative;
        background-color: #fff;
        box-sizing: border-box;
        margin-left: auto;
    }

    .dayview-grid-container {
        flex: 1 1 0;
        display: flex;
        align-items: flex-start;
    }

    .dayview-grid {
        border-bottom: #dadce0 1px solid;
        position: relative;
        min-width: 100%;
        flex: none;
        display: inline-flex;
        vertical-align: top;
    }

    .dayview-grid-tiles {
        z-index: 1;
        border-top: #dadce0 1px solid;
    }

    .dayview-grid-tile {
        height: 60px;
    }

    .dayview-grid-tile:after {
        content: "";
        border-bottom: #dadce0 1px solid;
        position: absolute;
        width: 100%;
        margin-top: -1px;
        z-index: 3;
        pointer-events: none;
    }

    .dayview-grid-marker-start,
    .dayview-grid-marker-end {
        width: 8px;
        border-inline-end: #dadce0 1px solid;
    }

    .dayview-grid-marker-end {
        position: absolute;
        left: 0;
        top: 0;
        bottom: 0;
    }

    .dayview-gridcell-container {
        position: relative;
        padding: 0 12px;
        box-sizing: border-box;
        flex: 1 0 auto;
        width: 129px;
        min-width: 129px;
        border-right: white 1px solid;
        overflow: visible;
    }

    .dayview-gridcell {
        grid-column-gap: 3px;
        z-index: 2;
        position: relative;
        height: 100%;
        width: 100%;
        display: grid;
        grid-template-rows: repeat(1440, 1px);
        /* grid-template-columns: fit-content(100%); */
    }

    .dayview-cell {
        z-index: 2;
        border-radius: 5px;
        border: 1px solid #80ff91;
        background-color: #80ffbf;
        padding: 0 3px;
    }

    .dayview-now-marker {
        position: absolute;
        z-index: 504;
        border-top: #ea4335 solid 2px;
        right: 10px;
        left: 8px;
        pointer-events: none;
        width: 96%;
    }

    .dayview-now-marker:after {
        background: #ea4335;
        border-radius: 50%;
        content: "";
        position: absolute;
        height: 12px;
        margin-inline-start: -6.5px;
        margin-top: -7px;
        width: 12px;
        left: 100%;
    }

    .dayview-cell {
        padding: 3px;
        color: white;
        font-size: 12px;
        display: flex;
    }

    .dayview-cell:not(.dayview-cell-extended) .dayview-cell-title:after {
        content: ",";
        margin-inline-end: 4px;
    }

    .dayview-cell-extended {
        display: block;
    }

    .dayview-cell-desc {
        display: none;
    }

    .dayview-cell-title,
    .dayview-cell-desc {
        white-space: normal;
        overflow-wrap: break-word;
        word-wrap: break-word;
    }

    .scrollable {
        height:70vh;
        overflow-y: scroll;
    }
</style>