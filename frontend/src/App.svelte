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
    // select "front cover" photo
    //
    // + button for like "i saw a car"
    //     - ask for location
    //     - "is it these" local cars (tick box: show all car)
    //     - if no, add new car + sighting at once
    //     - if yes, just add new sighting to exsiting car
    //
    // TODO code
    // switch to own OSM tile source (or, realistically in the short term, credit the one I am using)

    import { map, latLng, tileLayer, type MapOptions, Map, Util } from "leaflet";
    import "leaflet/dist/leaflet.css";
    import { onMount, tick } from "svelte";
    import CatInfo from "./lib/CatInfo.svelte";
    import CatProfile from "./lib/CatProfile.svelte";
    import SightingImages from "./lib/SightingImages.svelte";
    import { CatSighting, Cat } from "./lib/cat";
    import "./lib/popup.css";

    let cats: Cat[] = $state([]);
    let mymap: Map | undefined;
    onMount(() => {
        const options: MapOptions = {
            center: latLng(55.94170394241303, -3.1952485473196126),
            zoom: 16
        };
        mymap = map("map", options);
        mymap.addEventListener("click", () => { cats.map((c) => c.deselect_all()); cats = cats; selected_cat = undefined; });
        cats = [
            new Cat({
                id: 0,
                name: "Cat1",
                colour: "Black",
                markings: undefined,
                collar: "says CAT",
                description: "Nice cat seen near null island",
                sightings: [
                    new CatSighting({
                        pos: latLng(55.94170394241303, -3.1952485473196126),
                        who: "martyna",
                        when: 1745888711.9800618,
                        image_urls: ["/catmap/th-2346172708.jpeg"],
                        notes: undefined,
                        friendliness: 4,
                    }, mymap),
                    new CatSighting({
                        pos: latLng(55.94270394241303, -3.1852485473196126),
                        who: "martyna",
                        when: 1745880711.9800618,
                        image_urls: ["/catmap/th-2102865096.jpeg", "/catmap/th-2600831414.jpeg", "/catmap/th-448461832.jpeg", "/catmap/th-4145515264.jpeg", "/catmap/th-551657236.jpeg"],
                        notes: undefined,
                        friendliness: 4,
                    }, mymap),
                ]
            }),
            new Cat({
                id: 1,
                name: "Cat2",
                colour: "White",
                markings: "black splodge underside",
                collar: undefined,
                description: "a bit scaredy",
                sightings: [
                    new CatSighting({
                        pos: latLng(55.94180394241303, -3.1952285473196126),
                        who: "james",
                        when: 1745889711.9800618,
                        image_urls: [],
                        notes: undefined,
                        friendliness: 2,
                    }, mymap),
                ]
            })
        ];
        for (const c of cats) {
            for (const s of c.sightings) {
                s.marker.addEventListener("click", (_) => {
                    select_cat(c);
                });
            }
        }
    
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

    let add_cat = $state(false);
    function add_cat_button() {
        add_cat = true;
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
    let selected_sightings: CatSighting[] = $state([]);
    let positions: [number, number][] = $state([]);
    $effect(() => {
        if (selected_cat === undefined) {
            selected_sightings = [];
        } else {
            selected_sightings = selected_cat.sightings;
        }
    });

    $effect(() => { positions = selected_sightings.map((s) => s.ss_pos); });
</script>

<div id="root">
  <div id="map"></div>
  <div id="cat-list">
    {#each cats as cat}
      <CatInfo
        {cat}
        clicked={() => select_cat(cat)}
        showmore={() => (more_info = cat)}
      />
    {/each}
    <button id="add-cat-button" onclick={add_cat_button}>Add cat</button>
  </div>
</div>

<!--
{#each selected_sightings as s, i}
  <SightingImages sighting={s} pos={positions[i]} />
{/each}
-->

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

{#if more_info !== undefined}
  <CatProfile cat={more_info} />
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
    gap: 8px;
  }
  #add-cat-button {
    width: 100%;
    margin-top: auto;
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
