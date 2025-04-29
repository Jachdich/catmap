<script lang="ts">
    // TODO !IMPORTANT
    // last seen
    // where people have last seen this cat, territory
    // when adding new cat, ask "is this your car" for every cat nearby
    // edit existing cat while adding sighing?
    // list of cats, show their sightings
    // should colour be enum or free text
    // click on sighting, shows other sightings of same cat highlighted
    // avg friendliness based on each sighting
    //
    // + button for like "i saw a car"
    //     - ask for location
    //     - "is it these" local cars (tick box: show all car)
    //     - if no, add new car + sighting at once
    //     - if yes, just add new sighting to exsiting car
    //
    // TODO code
    // switch to own OSM tile source (or, realistically in the short term, credit the one I am using)

    import { map, latLng, tileLayer, marker, type MapOptions, Map } from "leaflet";
    import "leaflet/dist/leaflet.css";
    import { onMount } from "svelte";
    import CatInfo from "./lib/CatInfo.svelte";
    import { type Cat } from "./lib/cat";

    let cats: Cat[] = [
        {
            name: "Cat1",
            colour: "Black",
            markings: undefined,
            collar: "says CAT",
            description: "Nice cat seen near null island",
            friendliness: 4,
            sightings: [
                {
                    pos: latLng(55.94170394241303, -3.1952485473196126),
                    who: "james",
                    when: 1745888711.9800618,
                    image_urls: []
                }
            ]
        },
        {
            name: "Cat2",
            colour: "White",
            markings: "black splodge underside",
            collar: undefined,
            description: "a bit scaredy",
            friendliness: 2,
            sightings: [
                {
                    pos: latLng(55.94180394241303, -3.1952285473196126),
                    who: "james",
                    when: 1745889711.9800618,
                    image_urls: []
                }
            ]
        }
    ];
    let mymap: Map | undefined;

    onMount(() => {
        const options: MapOptions = {
            center: latLng(55.94170394241303, -3.1952485473196126),
            zoom: 16
        };
        mymap = map("map", options);
        console.log(mymap);

        tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        }).addTo(mymap);

        marker([51.5, -0.09]).addTo(mymap);
    });

    let add_cat = false;
    function add_cat_button() {
        add_cat = true;
    }
</script>

<div id="root">
    <div id="map"></div>
    <div id="cat-list">
        {#each cats as cat}
            <CatInfo {cat} />
        {/each}
        <button id="add-cat-button" on:click={add_cat_button}>Add cat</button>
    </div>
</div>
{#if add_cat}
    <div id="error" class="popup centre-window">
        <h2>New Sighting</h2>
        <div class="input-container">
            <p>hi</p>
            <input />
        </div>
        <div class="input-container">
            <p>hi</p>
            <p>hi2</p>
        </div>
        <div class="input-container">
            <p>hi</p>
            <p>hi2</p>
        </div>
    </div>
{/if}

<style>
    #root {
        display: flex;
        flex-direction: row;
        position: absolute;
        width: 100%;
        height: 100%;
    }
    #map {
        width: 100%;
        height: 100%;
        z-index: 1;
    }
    #cat-list {
        width: 400px;
        display: flex;
        flex-direction: column;
        height: 100%;
    }
    #add-cat-button {
        width: 100%;
        margin-top: auto;
    }
    .centre-window {
        display: grid;
        padding: 10px;
        border-bottom: 3px solid var(--panel-3);
        background-color: var(--panel-2);
        border-radius: var(--radius-3);
        position: absolute;
        width: max-content;
        left: 50%;
        right: 50%;
        top: 50%;
        -webkit-transform: translate(-50%, -50%);
        transform: translate(-50%, -50%);
    }

    .popup {
        z-index: 4;
    }

    .input-container {
        display: flex;
        justify-content: center;
        align-items: stretch;
        flex-direction: row;
        margin-bottom: 8px;
        margin-top: 8px;
    }

    :root {
        /* colours */
        --panel-0: #1e2329;
        --panel-1: #272e39;
        --panel-2: #31363f;
        --panel-3: #3c424f;

        --accent-1-light: #be8fd0;
        --accent-1-dark: #a776bb;

        --white-1: #d3d3d3;

        --text-dark: #1b0a24;
        --text-gray: #929292;

        /* radii & margins */
        /* intended to function well together */
        /* outer r = inner r + margin */
        --radius-1: 36px;
        --margin-1: 20px;
        --radius-2: 16px;
        --margin-2: 10px;
        --radius-3: 6px;

        background-color: var(--panel-0);
        font-family: sans-serif;
        color: var(--white-1);
    }

    input,
    button {
        border: 1px none;
        border-radius: var(--radius-3);
        color: var(--white-1);
        background-color: var(--panel-1);
    }

    input:focus {
        outline: none;
    }
</style>
