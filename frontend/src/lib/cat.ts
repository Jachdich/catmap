import { LatLng, marker, Marker, Map } from "leaflet";
import { cat_icon, cat_icon_sel } from "./icons";
import { tick } from "svelte";

type CatColour = "Black" | "White";
export class CatSighting {
    pos: LatLng;
    who: string; // TODO... (optional)
    when: number;
    friendliness: number;
    notes: string; // optional TODO
    image_urls: string[];
    marker: Marker;

    constructor(data: {
        pos: LatLng;
        who: string;
        when: number;
        image_urls: string[];
    }, map: Map) {
        this.pos = data.pos;
        this.who = data.who;
        this.when = data.when;
        this.image_urls = data.image_urls;

        this.marker = marker(this.pos, {icon: cat_icon});
        this.marker.addTo(map);
    }
}

// Kinda stupid these need to be classes. Maybe change that.
export class Cat {
    id: number;
    sightings: CatSighting[];
    name: string; // first person to see the cat names it, no editing 
    colour: CatColour;
    markings: string | undefined;
    collar: string | undefined;
    description: string;
    friendliness: number;
    selected: boolean = false;
    constructor(data: {
        id: number;
        sightings: CatSighting[];
        name: string;
        colour: CatColour;
        markings: string | undefined;
        collar: string | undefined;
        description: string;
        friendliness: number;
    }) {
        this.id = data.id;
        this.sightings = data.sightings; 
        this.name = data.name; 
        this.colour = data.colour; 
        this.markings = data.markings; 
        this.collar = data.collar; 
        this.description = data.description; 
        this.friendliness = data.friendliness;
    }

    select_sighting(s: CatSighting) {
        for (const s of this.sightings) {
            s.marker.setIcon(cat_icon_sel);
        }
    }
    deselect_all() {
        for (const s of this.sightings) {
            s.marker.setIcon(cat_icon);
        }
    }
}
