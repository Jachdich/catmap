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

    import { map, latLng, tileLayer, type MapOptions, Map, Util, type LeafletMouseEvent, LatLng, Popup, marker, Marker } from "leaflet";
    import "leaflet/dist/leaflet.css";
    import { onMount, tick } from "svelte";
    import CatInfo from "./lib/CatInfo.svelte";
    import CatProfile from "./lib/CatProfile.svelte";
    import SightingImages from "./lib/SightingImages.svelte";
    import { CatSighting, Cat } from "./lib/cat";
    import "./lib/popup.css";
    import { cat_icon_sel } from "./lib/icons";
    import MaybeNoneEditable from "./lib/MaybeNoneEditable.svelte";

    async function get_all_cats(): Promise<Cat[]> {
        const url = "http://localhost/api/v1/list_cats";
        const result = await fetch(url);
        const json: any[] = await result.json();
        let cats = [];
        for (const cat_id of json) {
            const cat = await get_cat(cat_id);
            if (cat !== undefined) {
                cats.push(cat);
            }
        }
        return cats;
    }

    async function get_cat(id: number): Promise<Cat | undefined> {
        const url = `http://localhost/api/v1/cat/${id}`;
        const result = await fetch(url);
        const json = await result.json()
        console.log(json);

        if (mymap !== undefined) {
            return Cat.from_json(json, mymap);
        }
        return undefined;
    }

    let cats: Cat[] = $state([]);
    let mymap: Map | undefined;
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
            if (add_cat == "Position") {
                if (mymap === undefined) return;
                if (add_cat_popup !== undefined) {
                    add_cat_popup.remove();
                }
                add_cat_pos = e.latlng;
                add_cat_popup = marker(add_cat_pos, {icon: cat_icon_sel});
                add_cat_popup.addTo(mymap);
            }
        });
        // cats = [
        //     new Cat({
        //         id: 0,
        //         name: "Cat1",
        //         colour: "Black",
        //         markings: undefined,
        //         collar: "says CAT",
        //         description: "Nice cat seen near null island",
        //         sightings: [
        //             new CatSighting({
        //                 pos: latLng(55.94170394241303, -3.1952485473196126),
        //                 who: "martyna",
        //                 when: new Date(1745888711.9800618),
        //                 image_urls: ["/catmap/th-2346172708.jpeg"],
        //                 notes: undefined,
        //                 friendliness: 4,
        //             }, mymap),
        //             new CatSighting({
        //                 pos: latLng(55.94270394241303, -3.1852485473196126),
        //                 who: "martyna",
        //                 when: new Date(1745880711.9800618),
        //                 image_urls: ["/catmap/th-2102865096.jpeg", "/catmap/th-2600831414.jpeg", "/catmap/th-448461832.jpeg", "/catmap/th-4145515264.jpeg", "/catmap/th-551657236.jpeg"],
        //                 notes: undefined,
        //                 friendliness: 4,
        //             }, mymap),
        //         ]
        //     }),
        //     new Cat({
        //         id: 1,
        //         name: "Cat2",
        //         colour: "White",
        //         markings: "black splodge underside",
        //         collar: undefined,
        //         description: "a bit scaredy",
        //         sightings: [
        //             new CatSighting({
        //                 pos: latLng(55.94180394241303, -3.1952285473196126),
        //                 who: "james",
        //                 when: new Date(1745889711.9800618),
        //                 image_urls: [],
        //                 notes: undefined,
        //                 friendliness: 2,
        //             }, mymap),
        //         ]
        //     })
        // ];
        get_all_cats().then(fetched_cats => {
            cats = fetched_cats;
            for (const c of cats) {
                for (const s of c.sightings) {
                    s.marker.addEventListener("click", (_) => {
                        select_cat(c);
                    });
                }
            }
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
    type AddState = "Not" | "Position" | "CheckNearby" | "AddSighting" | "NewCat";
    let add_cat: AddState = $state("Not");
    let add_cat_pos: LatLng | undefined = $state(undefined);
    let add_cat_popup: Marker | undefined = undefined;
    let nearby_cats: Cat[] = $state([]);
    let new_cat: Cat | undefined = $state(undefined);
    function add_cat_button() {
        add_cat = "Position";
    }

    function add_cat_selected_position() {
        add_cat = "CheckNearby";
        const pos = add_cat_pos as LatLng; // We know it must be not undefined
        nearby_cats = [];
        for (const cat of cats) {
            for (const sighting of cat.sightings) {
                if (sighting.pos.distanceTo(pos) < 1000) {
                    nearby_cats.push(cat);
                    break;
                }
            }
        }
    }

    function cancel_add_cat() {
        add_cat = "Not";
        nearby_cats = [];
        add_cat_pos = undefined;
        if (add_cat_popup !== undefined) {
            add_cat_popup.remove();
            add_cat_popup = undefined;
        }
    }

    function add_new_cat() {
        if (!mymap) return;
        add_cat = "NewCat";
        new_cat = new Cat({
        id: cats.length == 0 ? 0 : cats[cats.length - 1].id,
            sightings: [
                new CatSighting({
                    pos: add_cat_pos as LatLng, // Should exist by now
                    who: undefined,
                    when: new Date(),
                    image_urls: [],
                    friendliness: undefined,
                    notes: undefined}, mymap
                ),
            ],
            name: "",
            colour: "",
            markings: undefined,
            collar: undefined,
            description: "",
            best_image: undefined,
           
        });
    }

    function validate_friendliness(a: HTMLInputElement) {
        a.value = a.value.replace(/[^0-9]/g, '');
        if (a.value != "" && Number.parseInt(a.value) > 5) {
            a.value = "5";
        }
        if (a.value != "" && Number.parseInt(a.value) < 1) {
            a.value = "1";
        }
        if (new_cat !== undefined)
            new_cat.sightings[0].friendliness = Number.parseInt(a.value);
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

    function actually_add_cat() {
        if (new_cat === undefined) return;
        console.log(new_cat);
        new_cat.sightings[0].who = "me";
        let json = new_cat.to_json();
        fetch("http://localhost/api/v1/cats", {
          method: "POST",
          body: JSON.stringify(json),
          headers: {
            "Content-type": "application/json; charset=UTF-8"
          }
        });
        get_all_cats().then(new_cats => cats = new_cats);
        cancel_add_cat();
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
    <button id="add-cat-button" onclick={add_cat_button} disabled={add_cat != "Not"}>Add cat</button>
  </div>
</div>

<!--
{#each selected_sightings as s, i}
  <SightingImages sighting={s} pos={positions[i]} />
{/each}
-->

{#if add_cat == "Position"}
  <div id="catpos">
      <p>Select a position</p>
      <button type="button" class="position-done-button" disabled={add_cat_pos === undefined} onclick={add_cat_selected_position}>Done</button>
      <button type="button" class="position-done-button" onclick={cancel_add_cat}>Cancel</button>
  </div>
{/if}

{#if add_cat == "CheckNearby"}
  <div id="is-this-ur-car" class="popup centre-window">
    <div id="ur-car-scroll">
      <h2 id="title">Are any of these your car?</h2>
      <p id="subtitle">Check nearby cats in case your cat has already been seen</p>
      {#each nearby_cats as cat}
        <CatInfo {cat} clicked={function() {}} showmore={() => {more_info = cat}}/> 
      {/each}
    </div>
    <div class="bottom-buttons">
      <button type="button" class="button-expand-width" onclick={add_new_cat}>New cat</button>
      <button type="button" class="button-expand-width" onclick={cancel_add_cat}>Cancel</button>
    </div>
  </div>
{/if}

{#if add_cat == "NewCat" && new_cat !== undefined}
    <form id="new-cat" class="popup centre-window">
        <h2>Name</h2>
        <MaybeNoneEditable editing={true} val={new_cat.name} />
        <h2>Colour</h2>
        <MaybeNoneEditable editing={true} val={new_cat.colour} />
        <h2>Distinctive Markings</h2>
        <MaybeNoneEditable editing={true} val={new_cat.markings} />
        <h2>Collar</h2>
        <MaybeNoneEditable editing={true} val={new_cat.collar} />
        <h2>Description</h2>
        <MaybeNoneEditable editing={true} val={new_cat.description} />
        <h2>Friendliness</h2>
        <input oninput={(e) => validate_friendliness(e.currentTarget)} />
        <h2>Date seen</h2>
        <input type="date" value={new_cat.sightings[0].when} />
        <h2>Sighting notes</h2>
        <MaybeNoneEditable editing={true} val={new_cat.sightings[0].notes} />
        <h2>Pictures</h2>
        <input type="file" multiple />
        <div class="bottom-buttons">
            <button type="button" class="button-expand-width" onclick={actually_add_cat}>Submit</button>
            <button type="button" class="button-expand-width" onclick={cancel_add_cat}>Cancel</button>
        </div>
    </form>
{/if}

{#if more_info !== undefined}
  <CatProfile cat={more_info} close={() => more_info = undefined} />
{/if}

<style>

  #new-cat {
    display: flex;
    flex-direction: column;
  }
    
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
  .bottom-buttons {
    display: flex;
    flex-direction: row;
    margin-top: auto;
  }
  .button-expand-width {
    flex-grow: 1;
    margin-left: 4px;
    margin-right: 4px;
  }

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
  .input-container {
    display: flex;
    justify-content: center;
    align-items: stretch;
    flex-direction: row;
    margin-bottom: 8px;
    margin-top: 8px;
  }

  #catpos {
    z-index: 9;
    position: absolute;
    top: 16px;
    left: 50%;
    transform: translate(-50%, 0);
    background-color: var(--panel-0);
    display: flex;
    flex-direction: row;
    padding-left: 16px;
    padding-right: 16px;
    border-radius: 6px;
  }

  .position-done-button {
    height: fit-content;
    margin: auto;
    margin-left: 16px;
    padding-left: 8px;
    padding-right: 8px;
  }

</style>
