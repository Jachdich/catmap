<script lang="ts">
    import type Cat from "./cat";
    export let cat: Cat;
    function get_friendliness_desc(friendliness: number): string {
        return ["Runs away", "Keeps a safe distance", "Indifferent", "Curious", "Will approach you"][friendliness - 1];
    }
</script>

<div id="info_box">
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
</div>

<style>
    #info_box {
        background-color: #444444;
        color: #dddddd;
        padding: 6px;
        border-radius: 8px;
    }
</style>
