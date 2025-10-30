<script lang="ts">
    import "./top-buttons.css"
    import { Map, type LeafletMouseEvent, LatLng, marker, Marker } from "leaflet";
    import { cat_icon_sel } from "./icons";
    let add_cat_pos: LatLng | undefined = $state(undefined);
    let add_cat_popup: Marker | undefined = undefined;

    interface Props {
        mymap: Map,
        oncancel: () => void,
        ondone: (pos: LatLng) => void,
        mouse_event: (e: LeafletMouseEvent) => void,
    }

    let { mymap, oncancel, ondone, mouse_event = $bindable() }: Props = $props();
    
    mouse_event;
    mouse_event = function(e: LeafletMouseEvent) {
        if (mymap === undefined) return;
        if (add_cat_popup !== undefined) {
            add_cat_popup.remove();
        }
        add_cat_pos = e.latlng;
        add_cat_popup = marker(add_cat_pos, {icon: cat_icon_sel});
        add_cat_popup.addTo(mymap);
    }

    function cancel() {
        oncancel();
        add_cat_pos = undefined;
        if (add_cat_popup !== undefined) {
            add_cat_popup.remove();
            add_cat_popup = undefined;
        }
    }

    function done() {
        ondone(add_cat_pos as LatLng);

        add_cat_pos = undefined;
        if (add_cat_popup !== undefined) {
            add_cat_popup.remove();
            add_cat_popup = undefined;
        }
    }
</script>

<div class="top-buttons-container">
  <p>Select a position</p>
  <button type="button" class="top-button" disabled={add_cat_pos === undefined} onclick={done}>Done</button>
  <button type="button" class="top-button" onclick={cancel}>Cancel</button>
</div>
