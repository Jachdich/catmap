<script lang="ts">
    import Cropper from "svelte-easy-crop";
    import "./top-buttons.css";
    
    interface CropArea { x: number, y: number, width: number, height: number };

    let crop_done: ((value: CropArea) => void) | undefined = undefined;
    let crop_discard: (() => void) | undefined = undefined;

    let crop = $state({x: 0, y: 0});
    let zoom = $state(1);
    let crop_image: string | undefined = $state(undefined);
    let crop_area: CropArea = {x: 0, y: 0, width: 0, height: 0};
    let crop_show = $state(false);

    function readFile(file: File): Promise<string> {
        return new Promise((resolve, reject) => {
            let fr = new FileReader();
            fr.onload = _ => resolve(fr.result as string);
            fr.onerror = reject;
            fr.readAsDataURL(file)
        })
    }

    function get_crop(url: string): Promise<CropArea | undefined> {
        crop_image = url;
        crop_show = true;
        return new Promise((resolve, _) => {
            crop_done = (f) => {
                crop_show = false;
                resolve(f);
            }
            crop_discard = () => {
                crop_show = false;
                resolve(undefined);
            }
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

    export async function crop_images(image_files: FileList): Promise<Blob[]> {
        if (image_files === undefined) return [];
        const blobs: Blob[] = [];
        for (const f of image_files) {
            const url = await readFile(f);
            if (url == null) continue;
            const crop_area = await get_crop(url);
            if (crop_area === undefined) {
                continue;
            }
            const img = new Image();
            await set_image_url(img, url);
            const canvas = document.createElement("canvas");
            canvas.width = Math.min(crop_area.width, 1280); // Limit the size to something sensible
            canvas.height = Math.min(crop_area.height, 1280);
            const ctx = canvas.getContext("2d");
            if (!ctx) { return []; } // this shouldn't happen ???
            ctx.drawImage(img, crop_area.x, crop_area.y, crop_area.width, crop_area.height, 0, 0, canvas.width, canvas.height);
            const blob = await canvas_to_blob(canvas);
            if (blob !== null) {
                blobs.push(blob);
            }
        }
        return blobs;
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

    <div class="top-buttons-container">
      <p>Crop the image</p>
      <button type="button" class="top-button" onclick={() => { if (crop_done !== undefined) crop_done(crop_area); } }>Done</button>
      <button type="button" class="top-button" onclick={() => { if (crop_discard !== undefined) crop_discard(); } }>Cancel</button>
    </div>
{/if}

<style>
  #crop-container {
    position: fixed;
    z-index: 5;
    width: 100vw;
    height: 100vh;
    left: 50%;
    right: 50%;
    top: 50%;
    -webkit-transform: translate(-50%, -50%);
    transform: translate(-50%, -50%);
  }
</style>
