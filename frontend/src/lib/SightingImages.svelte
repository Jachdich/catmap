<script lang="ts">
    import { CatSighting } from "./cat";
    interface Props {
        sighting: CatSighting;
        pos: [number, number];
    }
    const { sighting, pos }: Props = $props();
    $effect(() => console.log(sighting.ss_pos));

</script>

<div id="imgbox" style="top: {pos[1]}px; left: {pos[0]}px;">
    {#each sighting.image_urls as image}
        <img src={image} alt="cat"/>
    {/each}
</div>

<style>
    #imgbox {
        display: flex;
        flex-direction: column;
        position: absolute;
    }

    img {
        width: 64px;
        height: auto;
        z-index: 5;
    }
</style>
