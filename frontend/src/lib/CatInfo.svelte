<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { Cat } from "./cat";
    const dispatch = createEventDispatcher();

    export let cat: Cat;
    function get_friendliness_desc(friendliness: number): string {
        return ["Runs away", "Keeps a safe distance", "Indifferent", "Curious", "Will approach you"][friendliness - 1];
    }
    let border = "2px solid #444444;";
    $: border = cat.selected ? "border: 2px solid white;" : "border: 2px solid #444444;";
    let thumb_url = "/catmap/catmeow.png";
    $: thumb_url = cat.sightings[0].image_urls.length > 0 ? cat.sightings[0].image_urls[0] : "/catmap/catmeow.png";
    function click() {
        dispatch("clicked");
    }

    let fr_str = "Unknown";
    $: {
        const fr = cat.friendliness();
        if (fr !== undefined) {
            fr_str = `${fr} (${get_friendliness_desc(fr)})`
        }
    }

    function more() {
        
    }
</script>

<div id="info-box" style={border}>
    <div id="info-button" >
        <img class="thumb" src={thumb_url} alt="The cat" />
        <a on:click={click} id="alink"><h2>{cat.name}</h2></a>
        <p>Colour: {cat.colour}</p>
        <p>Distinctive Markings:
            {#if cat.markings !== undefined}
                {cat.markings}
            {:else}
                <i>None</i>
            {/if}
        </p>
        <p>Collar:
            {#if cat.collar !== undefined}
                {cat.collar}
            {:else}
                <i>None</i>
            {/if}
        </p>
        <p>Description: {cat.description}</p>
        <p>Friendliness: {fr_str}</p>
        <p>Sightings: {cat.sightings.length}</p>﻿﻿
    </div>﻿
    <button type="button" on:click={more}>More</button>
</div>

<style>
    #info-box {
        background-color: #444444;
        color: #dddddd;
    }

    #info-button {
        background-color: #444444;
        color: #dddddd;
        text-align: left;
        font-style: normal;
        font-weight: 400;
        border: none;
        padding: 0px;
    }

    p {
        margin: 4px;
    }

    #alink {
        cursor: pointer;
        text-decoration: underline 2px;
    }

    .thumb {
        float: right;
        width: 128px;
    }
</style>
