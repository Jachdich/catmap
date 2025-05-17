<script lang="ts">
    import { Cat } from "./cat";

    interface Props {
        cat: Cat,
        clicked: () => void,
        showmore: () => void,
    }
    let { cat, clicked, showmore }: Props = $props();

    let border = $derived(cat.selected ? "border: 2px solid white;" : "border: 2px solid #444444;");
    let thumb_url = $derived(cat.sightings[0].image_urls.length > 0 ? cat.sightings[0].image_urls[0] : "/catmap/catmeow.png");
    function click() {
        clicked();
    }

    function more() {
        showmore();
    }
</script>

<div id="info-box" style={border}>
    <div id="info-button" >
        <img class="thumb" src={thumb_url} alt="The cat" />
        <button type="button" onclick={click} id="alink">{cat.name}</button>
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
        <p>Friendliness: {cat.friendliness() === undefined ? "Unknown" : `${cat.friendliness()} (${cat.friendliness_desc()})`}</p>
        <p>Sightings: {cat.sightings.length}</p>﻿﻿
    </div>﻿
    <button type="button" onclick={more}>More</button>
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
        background-color: #444444;
        color: #dddddd;
        border: none;
        font-size: 18px;
    }

    .thumb {
        float: right;
        width: 128px;
    }
</style>
