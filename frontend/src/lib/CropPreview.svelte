<script lang="ts">
    import MyCropper from "./MyCropper.svelte";

    interface Props {
        blobs: Blob[],
        mycropper: MyCropper,
    }

    let { blobs = $bindable(), mycropper }: Props = $props();

    async function crop(files: FileList) {
        blobs = await mycropper.crop_images(files);
    }
    
</script>

<input type="file" multiple accept="image/*" onchange={(e) => crop(e.currentTarget.files as FileList)}/>
<div id="preview-container">
    {#each blobs as image, i}
        <img src={URL.createObjectURL(image)} alt="Uploaded" class="preview-img" style="grid-row: {Math.floor(i / 3) + 1}; grid-column: {i % 3};"/>
    {/each}
</div>

<style>
    #preview-container {
        display: grid;
        gap: 4px;
        padding: 12px 0px;
    }

    .preview-img {
        width: 128px;
        border: 1px solid var(--white-1);
    }
</style>
