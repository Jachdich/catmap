import { LatLng, marker, Marker, Map, Popup } from "leaflet";
import { cat_icon, cat_icon_sel } from "./icons";
import { tick } from "svelte";

export interface SightingData {
    pos: LatLng;
    who: string | undefined;
    when: Date;
    image_urls: string[];
    friendliness: number | undefined;
    notes: string | undefined;
}

export class CatSighting {
    pos: LatLng;
    who: string | undefined;
    when: Date;
    friendliness: number | undefined;
    notes: string | undefined;
    image_urls: string[];
    marker: Marker;
    img_display_elem: HTMLDivElement | undefined;
    ss_pos: [number, number] = [0, 0];

    constructor(data: SightingData, map: Map) {
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
        p.options.autoPan = false;
        this.marker.bindPopup(p);
        this.marker.addTo(map);
    }

    static from_json(json: any, map: Map): CatSighting {
        return new CatSighting ({
            pos: new LatLng(json["pos"][0], json["pos"][1]),
            who: null_to_undefined(json["who"]),
            when: new Date(json["when"]),
            image_urls: json["image_urls"],
            friendliness: null_to_undefined(json["friendliness"]),
            notes: null_to_undefined(json["notes"]),
        }, map)
    }

    to_json(): any {
        return sighting_to_json(this);
    }
}

export function sighting_to_json(sighting: SightingData): any {
    return {
        pos: [sighting.pos.lat, sighting.pos.lng],
        who: sighting.who,
        when: Math.round(sighting.when.getTime() / 1000.0), // unix time ???
        image_urls: sighting.image_urls,
        friendliness: sighting.friendliness,
        notes: sighting.notes,
    }
}

export interface CatData {
    id: number;
    sightings: CatSighting[];
    name: string;
    colour: string;
    markings: string | undefined;
    collar: string | undefined;
    description: string | undefined;
    best_image: [number, number] | undefined;
}

function empty_to_undefined(s: string | undefined | null): string | undefined {
    if (s === undefined || s === null || s.trim() === "") return undefined;
    return s;
}

function null_to_undefined(x: any | null | undefined): any | undefined {
    if (x === null) return undefined;
    return x;
}

// Kinda stupid these need to be classes. Maybe change that.
// made stupider for the fact we need an interface too
export class Cat {
    id: number;
    sightings: CatSighting[];
    name: string; // first person to see the cat names it, no editing 
    colour: string;
    markings: string | undefined;
    collar: string | undefined;
    description: string | undefined;
    selected: boolean = false;
    best_image: [number, number] | undefined;
    constructor(data: CatData) {
        this.id = data.id;
        this.sightings = data.sightings; 
        this.name = data.name;
        this.colour = data.colour; 
        this.markings = empty_to_undefined(data.markings);
        this.collar = empty_to_undefined(data.collar);
        this.description = data.description; 
        this.best_image = data.best_image;

        // TODO temp
        if (this.best_image == undefined && this.sightings.length > 0 && this.sightings[0].image_urls.length > 0) {
            this.best_image = [0, 0];
        }
    }

    static from_json(json: any, map: Map): Cat {
        return new Cat({
            id: json["id"],
            sightings: json["sightings"].map((sighting_json: any) => CatSighting.from_json(sighting_json, map)),
            name: json["name"],
            colour: json["colour"],
            markings: null_to_undefined(json["markings"]),
            collar: null_to_undefined(json["collar"]),
            description: empty_to_undefined(json["description"]),
            best_image: null_to_undefined(json["best_image"]),
        })
    }

    to_json(): any {
        return {
            id: this.id,
            sightings: this.sightings.map(s => s.to_json()),
            name: this.name,
            colour: this.colour,
            markings: this.markings,
            collar: this.collar,
            description: this.description,
            best_image: this.best_image,
        }
    }

    get_best_image_url(): string | undefined {
        if (!this.best_image)
            return undefined;
        return this.sightings[this.best_image[0]].image_urls[this.best_image[1]];
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
