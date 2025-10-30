<script lang="ts">
    import { type Cat } from "./cat";
    import { type LatLng } from "leaflet";
    import CatInfo from "./CatInfo.svelte";

    interface Props {
        cats: Cat[],
        skip: () => void,
        onadd_sighting: (cat: Cat) => void,
        onadd_cat: () => void,
        onshow_more: (cat: Cat) => void,
        oncancel: () => void,
        pos: LatLng,
    }

    let { cats, skip, pos, onadd_sighting, onshow_more, oncancel, onadd_cat }: Props = $props();

    let nearby_cats: [Cat, number][] = $state([]);
    $effect(() => recalculate_nearby(cats));

    function recalculate_nearby(cats: Cat[]) {
        nearby_cats = [];
        for (const cat of cats) {
            for (const sighting of cat.sightings) {
                if (sighting.pos.distanceTo(pos) < 1000) {
                    let min_dist = Math.min(...cat.sightings.map(s => s.pos.distanceTo(pos)));
                    nearby_cats.push([cat, min_dist]);
                    break;
                }
            }
        }
        nearby_cats.sort((a, b) => {
            let a_min_dist = a[1];
            let b_min_dist = b[1];
            return a_min_dist - b_min_dist;
        });
        // Skip if there are no nearby cats
        if (nearby_cats.length == 0) {
            skip();
        }
    }

    function cancel() {
        oncancel();
        nearby_cats = [];
    }
</script>


<div id="is-this-ur-car" class="popup centre-window">
<div id="ur-car-scroll">
  <h2 id="title">Are any of these your car?</h2>
  <p id="subtitle">Check nearby cats in case your cat has already been seen</p>
  {#each nearby_cats as cat}
    <CatInfo cat={cat[0]} clicked={() => onadd_sighting(cat[0])} seen_dist={cat[1]} showmore={() => onshow_more(cat[0])}/> 
  {/each}
</div>
<div class="bottom-buttons">
  <button type="button" class="button-expand-width" onclick={onadd_cat}>New cat</button>
  <button type="button" class="button-expand-width" onclick={cancel}>Cancel</button>
</div>
</div>

<style>
  #title {
    margin-bottom: 0px;
  }
  #subtitle {
    margin-top: 0px;
  }
  #ur-car-scroll {
    overflow-y: scroll;
    display: flex;
    flex-direction: column;
    gap: 8px;
    background-color: inherit;
  }
  #is-this-ur-car {
    display: flex;
    flex-direction: column;
    gap: 4px;
    height: 80%;
    width: 80%;
    background-color: var(--panel-0);
    border: 2px solid var(--panel-2);
  }
</style>
