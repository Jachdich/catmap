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
</script>

<button type="button" id="info_box" style={border} on:click={click}>
    <img class="thumb" src={thumb_url} alt="The cat" />
    <p>{cat.name}</p>
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
    <p>Friendliness: {cat.friendliness} ({get_friendliness_desc(cat.friendliness)})</p>
    <p>Sightings: {cat.sightings.length}</p>
</button>

<style>
    #info_box {
        background-color: #444444;
        color: #dddddd;
        padding: 6px;
        text-align: left;
        font-style: normal;
        font-weight: 400;
    }

    p {
        margin: 4px;
    }

    .thumb {
        float: right;
        width: 128px;
    }
</style>
