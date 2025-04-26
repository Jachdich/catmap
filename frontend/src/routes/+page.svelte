<script lang="ts">
    // TODO !IMPORTANT
    // last seen
    // where people have last seen this cat, territory
    // when adding new cat, ask "is this your car" for every cat nearby
    // edit existing cat while adding sighing?
    // list of cats, show their sightings
    // should colour be enum or free text
    // click on sighting, shows other sightings of same cat highlighted
    import "cesium/Build/Cesium/Widgets/widgets.css";

    import { Cartesian3, Ion, Math as CesiumMath, Terrain, Viewer, Cartesian2, Cartographic, SceneMode, ImageryLayer, OpenStreetMapImageryProvider } from "cesium";
    import "cesium/Build/Cesium/Widgets/widgets.css";
    import { onMount } from "svelte";

    Ion.defaultAccessToken = import.meta.env.VITE_CESIUM_TOKEN;

    let cats: Cat[] = [];
    let viewer: Viewer | undefined;
    let ctx: CanvasRenderingContext2D | null = null;

    // to avoid instantiating objects continuously
    // may be premature optimisation but cesium does it so i will too
    const scratchc3_a: Cartesian3 = new Cartesian3();
    const scratchc3_b: Cartesian3 = new Cartesian3();

    onMount(() => {
        // Initialize the Cesium Viewer in the HTML element with the `cesiumContainer` ID.
        const osm = new OpenStreetMapImageryProvider({
            url : 'https://tile.openstreetmap.org/'
        });
        viewer = new Viewer("cesiumContainer", {
            terrain: Terrain.fromWorldTerrain(),
            baseLayerPicker: false,
            fullscreenButton: false,
            homeButton: false,
            sceneModePicker: false,
            timeline: false,
            animation: false,
            // selectionIndicator: false,
            navigationInstructionsInitiallyVisible: false,
            baseLayer: new ImageryLayer(osm),
            
        });
        ctx = canvas.getContext("2d");

        // Fly the camera to the origin longitude, latitude, and height.
        viewer.camera.flyTo({
            destination: Cartesian3.fromDegrees(-3.1952485473196126, 55.94170394241303, 5000),
            orientation: {
                heading: CesiumMath.toRadians(0.0),
                pitch: CesiumMath.toRadians(-90.0)
            },
            duration: 0
        });
        viewer.camera.switchToOrthographicFrustum();

        viewer.clock.onTick.addEventListener((_: any) => {
            if (viewer && ctx) {
                canvas.width = viewer.canvas.width;
                canvas.height = viewer.canvas.height;
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                for (const c of cats) {
                    for (const sighting_pos of c.sightings) {
                        draw_cat(sighting_pos, ctx, viewer);
                    }
                }
                if (info_box_div !== undefined && selected_cat !== undefined && viewer !== undefined) {
                    const pos = viewer.scene.cartesianToCanvasCoordinates(selected_cat.sightings[0]);
                    info_box_div.style.left = (pos.x + 8).toString() + "px";
                    info_box_div.style.top = (pos.y + 8).toString() + "px";
                }
            }
        });

        // Chrome doesn't support mouse events
        viewer.cesiumWidget.canvas.addEventListener("pointerdown", mousedown);
        viewer.cesiumWidget.canvas.addEventListener("pointerup", mouseup);
    });

    function draw_cat(c: Cartesian3, ctx: CanvasRenderingContext2D, viewer: Viewer) {
        const pos = viewer.scene.cartesianToCanvasCoordinates(c);
        const r = 10;
        const w = r/2;
        const h = r/2;
        if (pos.x >= -w && pos.y >= -h && pos.x < viewer.canvas.width + w && pos.y < viewer.canvas.height + h) {
            ctx.beginPath();
            ctx.ellipse(pos.x, pos.y, r, r, 0, 0, Math.PI * 2);
            ctx.fill();
        }
    }

    function mousedown(event: MouseEvent) {
        if (event.button == 2 && viewer !== undefined) {
            const cartesian = viewer.camera.pickEllipsoid(
                new Cartesian3(event.clientX, event.clientY),
                viewer.scene.ellipsoid
            );
            if (!cartesian) return; // just ignore an invalid position
            const cat = new Cat(cartesian);
            cats.push(cat);
            selected_cat = cat;
        }
    }

    function mouseup(_: MouseEvent) {
    }

    function keypress(event: KeyboardEvent) {}

    let mouseX: number, mouseY: number;
    function mousemove(event: MouseEvent) {
        mouseX = event.clientX;
        mouseY = event.clientY;
        // if (viewer !== undefined) {
        //     const ellipsoid = viewer.scene.ellipsoid;
        //     const cartesian = viewer.camera.pickEllipsoid(new Cartesian3(event.clientX, event.clientY), ellipsoid);
        //     if (!cartesian) return; // just ignore an invalid position
        // }
    }
    let canvas: HTMLCanvasElement;
    type CatColour = "Black" | "White";
    class CatSighting {
        pos: Cartesian3;
        who: string; // TODO...
        when: number;
    }
    class Cat {
        sightings: CatSighting[];
        name: string;
        colour: CatColour;
        markings: string | undefined;
        collar: string | undefined;
        description: string;
        image_urls: string[];
        friendliness: number;
        constructor(pos: Cartesian3) {
            this.sightings = [pos];
            this.name = "The Mysterious Cat of Piss"
            this.friendliness = 2;
        }
    }
    let selected_cat: Cat | undefined;
    let info_box_div: HTMLDivElement | undefined;

    function get_friendliness_desc(friendliness: number): string {
        return ["Runs away", "Keeps a safe distance", "Indifferent", "Curious", "Will approach you"][friendliness - 1];
    }
</script>

<canvas id="canvas" style="z-index: 2; position:absolute; pointer-events: none;" bind:this={canvas}></canvas>
<div id="cesiumContainer" style="height:max-content; z-index: 1;position:relative;"></div>
{#if selected_cat !== undefined}
    <div id="info_box" bind:this={info_box_div}>
        <p>{selected_cat.name}</p>
        <p>Colour: {selected_cat.colour}</p>
        <p>Distinctive Markings:
            {#if selected_cat.markings !== undefined}
                {selected_cat.markings}
            {:else}
                <i>None</i>
            {/if}
        </p>
        <p>Collar:
            {#if selected_cat.collar !== undefined}
                {selected_cat.collar}
            {:else}
                <i>None</i>
            {/if}
        </p>
        <p>Description: {selected_cat.description}</p>
        <p>Friendliness: {selected_cat.friendliness} ({get_friendliness_desc(selected_cat.friendliness)})</p>
        <p>Sightings: {selected_cat.sightings.length}</p>
    </div>
{/if}
<svelte:window on:keydown={keypress} on:mousemove|preventDefault={mousemove} />
<style>
    #info_box {
        position: absolute;
        z-index: 2;
        background-color: #444444;
        color: #dddddd;
        padding: 6px;
        border-radius: 8px;
    }
</style>
