<script lang="ts">
    import { map, latLng, tileLayer, type MapOptions, Map, Util, type LeafletMouseEvent, LatLng, Popup, marker, Marker } from "leaflet";
    import SightingImages from "./SightingImages.svelte";
    import MaybeNoneEditable from "./MaybeNoneEditable.svelte";
    import Cropper from "svelte-easy-crop";
    import { CatSighting, Cat, type CatData, type SightingData } from "./cat";
    import CatInfo from "./CatInfo.svelte";
    import { cat_icon_sel } from "./icons";

    interface Props {
        mymap: Map,
        cats: Cat[],
        onadded: () => void,
        oncancel: () => void,
        mouse_event: (e: LeafletMouseEvent) => void,
    }

    let { mymap, cats, onadded, oncancel, mouse_event = $bindable() }: Props = $props();

    interface CropArea { x: number, y: number, width: number, height: number };
    type State = "Position" | "CheckNearby" | "AddSighting" | "NewCat";
    let state_: State = $state("Position");
    let crop_show = $state(false);
    let add_cat_pos: LatLng | undefined = $state(undefined);
    let add_cat_popup: Marker | undefined = undefined;
    let nearby_cats: Cat[] = $state([]);
    let new_cat_data: CatData | undefined = $state(undefined);
    let new_sighting_data: SightingData | undefined = $state(undefined);
    let image_files: FileList | undefined = $state(undefined);
    let crop_done: ((value: CropArea) => void) | undefined = undefined;
    let crop_discard: (() => void) | undefined = undefined;

    let cropped_images: Blob[] = [];
    
    let crop = $state({x: 0, y: 0});
    let zoom = $state(1);
    let crop_image: string | undefined = $state(undefined);
    let crop_area: CropArea = {x: 0, y: 0, width: 0, height: 0};

    mouse_event = function(e: LeafletMouseEvent) {
        if (state_ == "Position") {
            if (mymap === undefined) return;
            if (add_cat_popup !== undefined) {
                add_cat_popup.remove();
            }
            add_cat_pos = e.latlng;
            add_cat_popup = marker(add_cat_pos, {icon: cat_icon_sel});
            add_cat_popup.addTo(mymap);
        }
    }
    
    function add_cat_selected_position() {
        state_ = "CheckNearby";
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
        // Skip if there are no nearby cats
        if (nearby_cats.length == 0) {
            add_new_cat();
        }
    }

    function cancel_add_cat() {
        oncancel();
        nearby_cats = [];
        add_cat_pos = undefined;
        if (add_cat_popup !== undefined) {
            add_cat_popup.remove();
            add_cat_popup = undefined;
        }
    }

    function add_new_cat() {
        state_ = "NewCat";
        new_cat_data = {
            id: cats.length == 0 ? 0 : cats[cats.length - 1].id,
            sightings: [],
            name: "",
            colour: "",
            markings: undefined,
            collar: undefined,
            description: "",
            best_image: undefined,
        };

        new_sighting_data = {
            pos: add_cat_pos as LatLng, // Should exist by now
            who: undefined,
            when: new Date(),
            image_urls: [],
            friendliness: undefined,
            notes: undefined,
        };
    }

    function validate_friendliness(a: HTMLInputElement) {
        a.value = a.value.replace(/[^0-9]/g, '');
        if (a.value != "" && Number.parseInt(a.value) > 5) {
            a.value = "5";
        }
        if (a.value != "" && Number.parseInt(a.value) < 1) {
            a.value = "1";
        }
        if (new_sighting_data !== undefined)
            new_sighting_data.friendliness = Number.parseInt(a.value);
    }

    async function actually_add_cat() {
        if (new_cat_data === undefined || new_sighting_data === undefined) return;

        let sighting_images = [];
        for (const blob of cropped_images) {
            let response = await fetch("http://localhost/api/v1/image", {
                method: "POST",
                headers: {"Content-Type": blob.type },
                body: blob,
            });
            let fname = await response.text();
            sighting_images.push(`http://localhost/api/v1/image/${fname}`);
        }
        new_sighting_data.image_urls = sighting_images;

        new_cat_data.sightings = [new CatSighting(new_sighting_data, mymap)];
        let cat = new Cat(new_cat_data);
        let json = cat.to_json();
        await fetch("http://localhost/api/v1/cat", {
          method: "POST",
          body: JSON.stringify(json),
          headers: {
            "Content-type": "application/json; charset=UTF-8"
          }
        });
        onadded();
        cancel_add_cat();

        image_files = undefined;
    }

    let new_cat_has_all_fields: boolean = $derived.by(() =>
        new_cat_data !== undefined &&
        new_sighting_data !== undefined &&
        new_cat_data.name !== undefined &&
        new_cat_data.name.trim() !== "" &&
        new_cat_data.colour !== undefined &&
        new_cat_data.colour.trim() !== "" &&
        new_sighting_data.when !== undefined
    );

    function readFile(file: File): Promise<string> {
        return new Promise((resolve, reject) => {
            let fr = new FileReader();
            fr.onload = _ => resolve(fr.result as string);
            fr.onerror = reject;
            fr.readAsDataURL(file)
        })
    }

    function get_crop(url: string): Promise<CropArea> {
        crop_image = url;
        return new Promise((resolve, reject) => {
            crop_done = resolve;
            crop_discard = reject;
        })
    }

    function canvas_to_blob(canvas: HTMLCanvasElement): Promise<Blob | null> {
        return new Promise((resolve, _) => 
            canvas.toBlob(function(blob) {
                resolve(blob);
            }, "image/jpeg")
        );
    }

    function set_image_url(image: HTMLImageElement, url: string): Promise<void> {
        return new Promise((resolve, _) => {
            image.onload = () => resolve();
            image.src = url;
        })
    }

    async function crop_images() {
        crop_show = true;
        if (image_files === undefined) return [];
        const blobs: Blob[] = [];
        for (const f of image_files) {
            const url = await readFile(f);
            if (url == null) continue;
            const crop_area = await get_crop(url);
            const img = new Image();
            await set_image_url(img, url);
            const canvas = document.createElement("canvas");
            canvas.width = 256;
            canvas.height = 256;
            const ctx = canvas.getContext("2d");
            if (!ctx) return []; // this shouldn't happen ???
            ctx.drawImage(img, crop_area.x, crop_area.y, crop_area.width, crop_area.height, 0, 0, 256, 256);
            const blob = await canvas_to_blob(canvas);
            if (blob !== null)
                blobs.push(blob);
        }
        crop_show = false;
        cropped_images = blobs;
    }

</script>

{#if crop_show}
<div id="crop-container">
<Cropper
    image={crop_image}
    bind:crop
    bind:zoom
    aspect={1}
    maxZoom={10}
    oncropcomplete={e => {
        crop_area = e.pixels;
    }}
/>
</div>
  <div class="top-done-buttons">
      <p>Crop the image</p>
      <button type="button" class="position-done-button" onclick={() => { if (crop_done !== undefined) crop_done(crop_area); } }>Done</button>
      <button type="button" class="position-done-button" onclick={() => { if (crop_discard !== undefined) crop_discard(); } }>Cancel</button>
  </div>
{/if}

{#if state_ == "Position"}
  <div class="top-done-buttons">
      <p>Select a position</p>
      <button type="button" class="position-done-button" disabled={add_cat_pos === undefined} onclick={add_cat_selected_position}>Done</button>
      <button type="button" class="position-done-button" onclick={cancel_add_cat}>Cancel</button>
  </div>
{/if}

{#if state_ == "CheckNearby"}
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

{#if state_ == "NewCat" && new_cat_data !== undefined && new_sighting_data !== undefined}
    <form id="new-cat" class="popup centre-window">
        <h2>Name<sup>*</sup></h2>
        <MaybeNoneEditable editing={true} bind:val={new_cat_data.name} />
        <h2>Colour<sup>*</sup></h2>
        <MaybeNoneEditable editing={true} bind:val={new_cat_data.colour} />
        <h2>Distinctive Markings</h2>
        <MaybeNoneEditable editing={true} bind:val={new_cat_data.markings} />
        <h2>Collar</h2>
        <MaybeNoneEditable editing={true} bind:val={new_cat_data.collar} />
        <h2>Description</h2>
        <MaybeNoneEditable editing={true} bind:val={new_cat_data.description} />
        <h2>Friendliness</h2>
        <input oninput={(e) => validate_friendliness(e.currentTarget)} />
        <h2>Date seen</h2>
        <input type="date" value={new_sighting_data.when.toISOString().slice(0,10)} oninput={(event: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
            if (new_sighting_data) new_sighting_data.when = new Date(event.currentTarget.value)
        }} />
        <h2>Sighting notes</h2>
        <MaybeNoneEditable editing={true} bind:val={new_sighting_data.notes} />
        <h2>Who saw it?</h2>
        <MaybeNoneEditable editing={true} bind:val={new_sighting_data.who} />
        <h2>Pictures</h2>
        <input type="file" multiple bind:files={image_files} onchange={crop_images}/>
        <div class="bottom-buttons">
            <button type="button" class="button-expand-width" onclick={actually_add_cat} disabled={!new_cat_has_all_fields}>Submit</button>
            <button type="button" class="button-expand-width" onclick={cancel_add_cat}>Cancel</button>
        </div>
    </form>
{/if}

<style>
    h2 {
        font-size: 14px;
    }

  #crop-container {
    position: absolute;
    z-index: 30;
    width: 80%;
    height: 80%;
    left: 50%;
    right: 50%;
    top: 50%;
    -webkit-transform: translate(-50%, -50%);
    transform: translate(-50%, -50%);
  }
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
  .input-container {
    display: flex;
    justify-content: center;
    align-items: stretch;
    flex-direction: row;
    margin-bottom: 8px;
    margin-top: 8px;
  }

  .top-done-buttons {
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
