import { LatLng, marker, Marker, Map, Popup } from "leaflet";
import { cat_icon, cat_icon_sel } from "./icons";
import { tick } from "svelte";

type CatColour = "Black" | "White";
export class CatSighting {
    pos: LatLng;
    who: string | undefined;
    when: number;
    friendliness: number | undefined;
    notes: string | undefined;
    image_urls: string[];
    marker: Marker;
    img_display_elem:HTMLDivElement | undefined;
    ss_pos: [number, number] = [0, 0];

    constructor(data: {
        pos: LatLng;
        who: string | undefined;
        when: number;
        image_urls: string[];
        friendliness: number | undefined;
        notes: string | undefined;
    }, map: Map) {
        this.pos = data.pos;
        this.who = data.who;
        this.when = data.when;
        this.image_urls = data.image_urls;
        this.friendliness = data.friendliness;
        this.notes = data.notes;

        this.marker = marker(this.pos, {icon: cat_icon});
        let content = '<div style="display: flex; flex-direction: column;">';
        for (const url of this.image_urls) {
            content += "<img src='" + url + "' style='width: 64px'/>";
        }
        content += "</div>"
        let p = new Popup().setContent(content);
        p.options.autoClose = false;
        this.marker.bindPopup(p);
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
    selected: boolean = false;
    constructor(data: {
        id: number;
        sightings: CatSighting[];
        name: string;
        colour: CatColour;
        markings: string | undefined;
        collar: string | undefined;
        description: string;
    }) {
        this.id = data.id;
        this.sightings = data.sightings; 
        this.name = data.name; 
        this.colour = data.colour; 
        this.markings = data.markings; 
        this.collar = data.collar; 
        this.description = data.description; 
        // this.friendliness = data.friendliness;
    }

    select_all() {
        for (const s of this.sightings) {
            s.marker.setIcon(cat_icon_sel);
            s.marker.openPopup();
        }
        this.selected = true;
    }
    deselect_all() {
        for (const s of this.sightings) {
            s.marker.setIcon(cat_icon);
            s.marker.closePopup();
        }
        this.selected = false;
    }

    friendliness(): number | undefined {
        let fs = this.sightings.filter((s) => s.friendliness !== undefined).map((s) => s.friendliness) as number[]; // we filtered out the undefineds
        if (fs.length == 0) return undefined;
        fs.sort();
        return fs[Math.floor(fs.length / 2)];
    }
    friendliness_desc(): string | undefined {
        const friendliness = this.friendliness();
        if (friendliness === undefined) {
            return;
        }
        return ["Runs away", "Keeps a safe distance", "Indifferent", "Curious", "Will approach you"][friendliness - 1];
    }
}
