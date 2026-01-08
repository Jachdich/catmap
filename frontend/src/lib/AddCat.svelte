<script lang="ts">
    import { Map, type LeafletMouseEvent, LatLng } from "leaflet";
    import MaybeNoneEditable from "./MaybeNoneEditable.svelte";
    import { CatSighting, Cat, type CatData, type SightingData, sighting_to_json } from "./cat";
    import PositionPrompt from "./PositionPrompt.svelte";
    import CheckNearby from "./CheckNearby.svelte";
    import AddSighting from "./AddSighting.svelte";
    import "./top-buttons.css"
    import CatProfile from "./CatProfile.svelte";
    import CropPreview from "./CropPreview.svelte";
    import MyCropper from "./MyCropper.svelte";

    interface Props {
        mymap: Map,
        cats: Cat[],
        onadded: () => void,
        oncancel: () => void,
        mouse_event: (e: LeafletMouseEvent) => void,
    }

    let { mymap, cats, onadded, oncancel, mouse_event = $bindable() }: Props = $props();

    type State = { tag: "Position" }
               | { tag: "CheckNearby", pos: LatLng }
               | { tag: "MoreInfo", pos: LatLng, cat: Cat }
               | { tag: "AddSighting", new_sighting_data: SightingData, cat_id: number, cat_name: string }
               | { tag: "NewCat", new_cat_data: CatData, new_sighting_data: SightingData };
    let state_: State = $state({ tag: "Position" });
    let show_dim_bg: boolean = $derived.by(() =>
        (  state_.tag === "CheckNearby"
        || state_.tag === "AddSighting"
        || state_.tag === "NewCat"
        || state_.tag === "MoreInfo"));
    let cropped_images: Blob[] = $state([]);

    function add_cat_selected_position(pos: LatLng) {
        state_ = { tag: "CheckNearby", pos: pos };
    }

    function cancel_add_cat() {
        oncancel();
        state_ = { tag: "Position" };
    }

    function add_new_cat() {
        if (state_.tag != "CheckNearby") {
            // invalid transition
            return;
        }
        state_ = {
            tag: "NewCat",
            new_cat_data: {
                id: cats.length == 0 ? 0 : cats[cats.length - 1].id,
                sightings: [],
                name: "",
                colour: "",
                markings: undefined,
                collar: undefined,
                description: "",
                best_image: undefined,
            },

            new_sighting_data: {
                pos: state_.pos,
                who: undefined,
                when: new Date(),
                image_urls: [],
                friendliness: undefined,
                notes: undefined,
            }
        };
    }

    // function draw_service_point_icon(x: number, y: number, scl: number, ctx: CanvasRenderingContext2D) {
        // ctx.strokeStyle = "#ffffff";
        // ctx.fillStyle = "#ffffff";
        // ctx.lineWidth = 2;
        // ctx.lineCap = "round";
        // ctx.beginPath();
        // const big_rad = 22;
        // const small_rad = 3;
        // const big_angle = 4.034677631835291;
        // const small_angle = 2.248507675344295;
        // ctx.arc(x, y - 50 * scl, big_rad * scl, big_angle/2 - Math.PI/2, -big_angle/2 - Math.PI/2, true);
        // const x0 = x + Math.cos(big_angle/2 - Math.PI/2) * big_rad * scl;
        // const y0 = y - 50 * scl + Math.sin(big_angle/2 - Math.PI/2) * big_rad * scl;
        // const x1 = x + Math.cos(-small_angle/2 + Math.PI/2) * small_rad * scl;
        // const y1 = y - small_rad * 2 * scl + Math.sin(-small_angle/2 + Math.PI/2) * small_rad * scl;
        // ctx.moveTo(x0, y0);
        // ctx.lineTo(x1, y1);
        // ctx.arc(x, y - small_rad * 2 * scl, small_rad * scl, small_angle/2 + Math.PI/2, -small_angle/2 + Math.PI/2, true);
        // ctx.moveTo(2*x - x0, y0);
        // ctx.lineTo(2*x - x1, y1);
        // ctx.fill();
        // ctx.stroke();
        // ctx.beginPath();
        // ctx.arc(x, y - 50 * scl, 17 * scl, 0, Math.PI * 2);

        // const dx = 11;
        // const dy = dx / 2;
        // const box_low = dx;
        // const box_vert = dx;
        // const bx = x - 5 * scl;

        // ctx.moveTo(bx, y - (50 - box_low) * scl);
        // ctx.lineTo(bx - dx * scl, y - (50 - box_low + dy) * scl);
        // ctx.lineTo(bx - dx * scl, y - (50 - box_low + box_vert + dy) * scl);
        // ctx.lineTo(bx, y - (50 - box_low + box_vert + 2*dy) * scl);
        // ctx.moveTo(bx - dx * scl, y - (50 - box_low + box_vert + dy) * scl);
        // ctx.lineTo(bx, y - (50 - box_low + box_vert) * scl);

        // ctx.moveTo(bx, y - (50 - box_low) * scl);
        // ctx.lineTo(bx + dx * scl, y - (50 - box_low + dy) * scl);
        // ctx.lineTo(bx + dx * scl, y - (50 - box_low + box_vert + dy) * scl);
        // ctx.lineTo(bx, y - (50 - box_low + box_vert + 2*dy) * scl);
        // ctx.moveTo(bx + dx * scl, y - (50 - box_low + box_vert + dy) * scl);
        // ctx.lineTo(bx, y - (50 - box_low + box_vert) * scl);

        // ctx.moveTo(bx, y - (50 - box_low) * scl);
        // ctx.lineTo(bx, y - (50 - box_low + box_vert) * scl);

        // ctx.moveTo(bx - dx / 2 * scl, y - (50 - box_low + dy / 2) * scl);
        // ctx.lineTo(bx - dx / 2 * scl, y - (50 - box_low + dy / 2 + box_vert) * scl);
        // ctx.lineTo(bx + dx / 2 * scl, y - (50 - box_low + dy * (3/2) + box_vert) * scl);

        // ctx.stroke();
    // }

    function validate_friendliness(a: HTMLInputElement) {
        a.value = a.value.replace(/[^0-9]/g, '');
        if (a.value != "" && Number.parseInt(a.value) > 5) {
            a.value = "5";
        }
        if (a.value != "" && Number.parseInt(a.value) < 1) {
            a.value = "1";
        }
        if (state_.tag === "NewCat" || state_.tag === "AddSighting")
            state_.new_sighting_data.friendliness = Number.parseInt(a.value);
    }

    async function actually_add_cat() {
        if (state_.tag !== "NewCat") {
            return;
        }

        let sighting_images = [];
        for (const blob of cropped_images || []) {
            let response = await fetch("/api/v1/image", {
                method: "POST",
                headers: {"Content-Type": blob.type },
                body: blob,
            });
            let fname = await response.text();
            sighting_images.push(`${fname}`);
        }
        state_.new_sighting_data.image_urls = sighting_images;

        state_.new_cat_data.sightings = [new CatSighting(state_.new_sighting_data, mymap)];
        let cat = new Cat(state_.new_cat_data);
        let json = cat.to_json();
        await fetch("/api/v1/cat", {
          method: "POST",
          body: JSON.stringify(json),
          headers: {
            "Content-type": "application/json; charset=UTF-8"
          }
        });
        onadded();
        cancel_add_cat();

    }

    async function actually_add_sighting() {
        if (state_.tag !== "AddSighting") {
            return;
        }

        let sighting_images = [];
        for (const blob of cropped_images || []) {
            let response = await fetch("/api/v1/image", {
                method: "POST",
                headers: {"Content-Type": blob.type },
                body: blob,
            });
            let fname = await response.text();
            sighting_images.push(`${fname}`);
        }
        state_.new_sighting_data.image_urls = sighting_images;

        let json = sighting_to_json(state_.new_sighting_data);
        await fetch(`/api/v1/cat/${state_.cat_id}/sightings`, {
          method: "POST",
          body: JSON.stringify(json),
          headers: {
            "Content-type": "application/json; charset=UTF-8"
          }
        });
        onadded();
        cancel_add_cat();

    }

    let new_cat_has_all_fields: boolean = $derived.by(() =>
        state_.tag === "NewCat" &&
        state_.new_cat_data !== undefined &&
        state_.new_sighting_data !== undefined &&
        state_.new_cat_data.name !== undefined &&
        state_.new_cat_data.name.trim() !== "" &&
        state_.new_cat_data.colour !== undefined &&
        state_.new_cat_data.colour.trim() !== "" &&
        state_.new_sighting_data.when !== undefined
    );


    function add_sighting(cat: Cat) {
        if (state_.tag != "CheckNearby") {
            // invalid transition
            return;
        }
        state_ = {
            tag: "AddSighting",
            cat_id: cat.id,
            cat_name: cat.name,
            new_sighting_data: {
                pos: state_.pos,
                who: undefined,
                when: new Date(),
                image_urls: [],
                friendliness: undefined,
                notes: undefined,
            }
        };
    }

    function show_more_info(cat: Cat) {
        if (state_.tag != "CheckNearby") {
            // invalid transition
            return;
        }
        state_ = { tag: "MoreInfo", cat: cat, pos: state_.pos };
    }

    function close_more_info() {
        if (state_.tag != "MoreInfo") {
            // invalid transition
            return;
        }

        state_ = { tag: "CheckNearby", pos: state_.pos };
    }

    let mycropper: MyCropper;
</script>

{#if state_.tag == "Position"}
    <PositionPrompt {mymap} bind:mouse_event={mouse_event} oncancel={cancel_add_cat} ondone={add_cat_selected_position} />
{/if}

{#if state_.tag == "CheckNearby"}
    <CheckNearby
        {cats}
        pos={state_.pos}
        skip={() => 1}
        onadd_sighting={add_sighting}
        onadd_cat={add_new_cat}
        onshow_more={show_more_info}
        oncancel={cancel_add_cat} />
{/if}

{#if state_.tag == "AddSighting"}
    <AddSighting bind:new_sighting_data={state_.new_sighting_data} oncancel={cancel_add_cat} cat_name={state_.cat_name} onsubmit={actually_add_sighting} bind:new_sighting_images={cropped_images} {mycropper}/>
{/if}

<MyCropper bind:this={mycropper} />

{#if state_.tag == "NewCat"}
    <form id="new-cat" class="popup centre-window">
        <h2>Name<sup>*</sup></h2>
        <MaybeNoneEditable editing={true} bind:val={state_.new_cat_data.name} />
        <h2>Colour<sup>*</sup></h2>
        <MaybeNoneEditable editing={true} bind:val={state_.new_cat_data.colour} />
        <h2>Distinctive Markings</h2>
        <MaybeNoneEditable editing={true} bind:val={state_.new_cat_data.markings} />
        <h2>Collar</h2>
        <MaybeNoneEditable editing={true} bind:val={state_.new_cat_data.collar} />
        <h2>Description</h2>
        <MaybeNoneEditable editing={true} bind:val={state_.new_cat_data.description} />
        <h2>Friendliness</h2>
        <input oninput={(e) => validate_friendliness(e.currentTarget)} />
        <h2>Date seen</h2>
        <input type="date" value={state_.new_sighting_data.when.toISOString().slice(0,10)} oninput={(event: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
            if (state_.tag === "NewCat") state_.new_sighting_data.when = new Date(event.currentTarget.value)
        }} />
        <h2>Sighting notes</h2>
        <MaybeNoneEditable editing={true} bind:val={state_.new_sighting_data.notes} />
        <h2>Who saw it?</h2>
        <MaybeNoneEditable editing={true} bind:val={state_.new_sighting_data.who} />
        <h2>Pictures</h2>
        <CropPreview bind:blobs={cropped_images} {mycropper} />
        <div class="bottom-buttons">
            <button type="button" class="button-expand-width" onclick={actually_add_cat} disabled={!new_cat_has_all_fields}>Submit</button>
            <button type="button" class="button-expand-width" onclick={cancel_add_cat}>Cancel</button>
        </div>
    </form>
{/if}

{#if state_.tag == "MoreInfo" }
    <CatProfile cat={state_.cat} close={close_more_info} edited={() => 1}/>
{/if}

{#if show_dim_bg}
    <div class="dim-bg"></div>
{/if}

<style>
    h2 {
        font-size: 14px;
    }

    .dim-bg {
        background-color: #00000080;
        position: absolute;
        z-index: 2;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
    }

  #new-cat {
    display: flex;
    flex-direction: column;
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
</style>
