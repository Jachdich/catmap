<script lang="ts">
    import { type SightingData } from "./cat";
    import CropPreview from "./CropPreview.svelte";
    import MaybeNoneEditable from "./MaybeNoneEditable.svelte";
    import MyCropper from "./MyCropper.svelte";
    interface Props {
        new_sighting_data: SightingData,
        new_sighting_images: Blob[],
        cat_name: string,
        onsubmit: () => void,
        oncancel: () => void,
        mycropper: MyCropper,
    }

    let { new_sighting_data = $bindable(), new_sighting_images = $bindable(), cat_name, onsubmit, oncancel, mycropper }: Props = $props();

    function validate_friendliness(a: HTMLInputElement) {
        a.value = a.value.replace(/[^0-9]/g, '');
        if (a.value != "" && Number.parseInt(a.value) > 5) {
            a.value = "5";
        }
        if (a.value != "" && Number.parseInt(a.value) < 1) {
            a.value = "1";
        }
        new_sighting_data.friendliness = Number.parseInt(a.value);
    }

</script>

<form id="new-cat" class="popup centre-window">
    <h1>Seen: {cat_name}</h1>
    <h2>Friendliness</h2>
    <input oninput={(e) => validate_friendliness(e.currentTarget)} />
    <h2>Date seen</h2>
    <input type="date" value={new_sighting_data.when.toISOString().slice(0,10)} oninput={(event: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
        new_sighting_data.when = new Date(event.currentTarget.value)
    }} />
    <h2>Sighting notes</h2>
    <MaybeNoneEditable editing={true} bind:val={new_sighting_data.notes} />
    <h2>Who saw it?</h2>
    <MaybeNoneEditable editing={true} bind:val={new_sighting_data.who} />
    <h2>Pictures</h2>

    <CropPreview bind:blobs={new_sighting_images} {mycropper} />
    
    <div class="bottom-buttons">
        <button type="button" class="button-expand-width" onclick={onsubmit}>Submit</button>
        <button type="button" class="button-expand-width" onclick={oncancel}>Cancel</button>
    </div>
</form>

<style>
    h2 {
        font-size: 14px;
    }

    h1 {
        font-size: 18px;
        margin: 0px auto;
    }
</style>
