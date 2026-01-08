<script lang="ts">
    // TODO !IMPORTANT
    // last seen
    // where people have last seen this cat, territory
    // edit existing cat while adding sighing?
    // should colour be enum or free text
    // select "front cover" photo
    //
    // + button for like "i saw a car"
    //     - ask for location
    //     - "is it these" local cars (tick box: show all car)
    //     - if no, add new car + sighting at once
    //     - if yes, just add new sighting to exsiting car
    //
    //
    // Bug: some kind of effect loop involving CheckNearby (think it's fixed - keep eye out)
    // bug: error for non-image sent to cropper
    // 

    import { map, latLng, tileLayer, type MapOptions, Map, type LeafletMouseEvent, marker } from "leaflet";
    import "leaflet/dist/leaflet.css";
    import { onMount } from "svelte";
    import CatInfo from "./lib/CatInfo.svelte";
    import CatProfile from "./lib/CatProfile.svelte";
    import { Cat } from "./lib/cat";
    import "./lib/popup.css";
    import AddCat from "./lib/AddCat.svelte";

    async function get_all_cats(): Promise<Cat[]> {
        const url = "/api/v1/list_cats";
        const result = await fetch(url);
        const json: any[] = await result.json();
        let cats = [];
        for (const cat_id of json) {
            const cat = await get_cat(cat_id);
            if (cat !== undefined) {
                cats.push(cat);
            }
        }
        for (const c of cats) {
            for (const s of c.sightings) {
                s.marker.addEventListener("click", (_) => {
                    select_cat(c);
                });
            }
        }
        return cats;
    }

    async function get_cat(id: number): Promise<Cat | undefined> {
        const url = `/api/v1/cat/${id}`;
        const result = await fetch(url);
        const json = await result.json();

        if (mymap !== undefined) {
            return Cat.from_json(json, mymap);
        }
        return undefined;
    }

    function select_cat(cat: Cat) {
        for (const c of cats) {
            c.deselect_all();
        }
        cat.select_all();
        cats = cats;
        selected_cat = cat;
    }

    let more_info: Cat | undefined = $state(undefined);
    let selected_cat: Cat | undefined = $state(undefined);
    // let selected_sightings: CatSighting[] = $state([]);
    // let positions: [number, number][] = $state([]);
    // $effect(() => {
    //     if (selected_cat === undefined) {
    //         selected_sightings = [];
    //     } else {
    //         selected_sightings = selected_cat.sightings;
    //     }
    // });

    // $effect(() => { positions = selected_sightings.map((s) => s.ss_pos); });

    let cats: Cat[] = $state([]);
    let mymap: Map | undefined = $state(undefined);
    onMount(() => {
        const options: MapOptions = {
            center: latLng(55.94170394241303, -3.1952485473196126),
            zoom: 16
        };
        mymap = map("map", options);
        mymap.addEventListener("click", (e: LeafletMouseEvent) => {
            if (selected_cat !== undefined) {
                cats.map((c) => c.deselect_all());
                cats = cats;
                selected_cat = undefined;
            }

            if (adding_new_cat) {
                add_cat_click_callback(e);
            }
        });
        get_all_cats().then(fetched_cats => {
            cats = fetched_cats;
        });
    
        tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        }).addTo(mymap);


        // mymap.addEventListener("move", update_image_positions);
        // mymap.addEventListener("update", update_image_positions);
        // Util.requestAnimFrame((_) => update_image_positions());

    });

    // function update_image_positions() {
    //     if (mymap === undefined) return;
    //     for (let i = 0; i < selected_sightings.length; i++) {
    //         let ss_pos = mymap.latLngToContainerPoint(selected_sightings[i].pos);
    //         positions[i] = [ss_pos.x, ss_pos.y];
    //     }
    //     positions = positions;
    //     Util.requestAnimFrame((_) => update_image_positions());
    // }
    let adding_new_cat: boolean = $state(false);
    let add_cat_click_callback: ((e: LeafletMouseEvent) => void) = $state((_) => undefined);


    async function cat_edited(cat: Cat) {
        let json = cat.to_json();
        json["sightings"] = []; // we're not editing the sightings
        await fetch(`/api/v1/cat/${cat.id}`, {
          method: "PUT",
          body: JSON.stringify(json),
          headers: {
            "Content-type": "application/json; charset=UTF-8"
          }
        });
    }
</script>

<div id="root">
  <div id="map"></div>
  <div id="right-bar">
    <div id="cat-list">
      {#each cats as cat}
        <CatInfo
          {cat}
          clicked={() => select_cat(cat)}
          showmore={() => (more_info = cat)}
        />
      {/each}
    </div>
    <button id="add-cat-button" onclick={() => adding_new_cat = true} disabled={adding_new_cat}>Add cat</button>
  </div>
</div>

<!--
{#each selected_sightings as s, i}
  <SightingImages sighting={s} pos={positions[i]} />
{/each}
-->

{#if mymap !== undefined && adding_new_cat}
    <AddCat mymap={mymap} cats={cats} bind:mouse_event={add_cat_click_callback} oncancel={() => adding_new_cat = false } onadded={() => get_all_cats().then(new_cats => cats = new_cats) } />
{/if}


{#if more_info !== undefined}
  <CatProfile cat={more_info} close={() => more_info = undefined} edited={cat_edited} />
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
  #right-bar {
    width: 400px;
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 4px;
  }
  #cat-list {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 8px;
    overflow-y: scroll;
  }
  #add-cat-button {
    margin: 4px;
    margin-top: auto;
  }
</style>
