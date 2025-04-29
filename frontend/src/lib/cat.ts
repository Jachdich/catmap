import { LatLng } from "leaflet";

type CatColour = "Black" | "White";
export interface CatSighting {
    pos: LatLng;
    who: string; // TODO...
    when: number;
    image_urls: string[];
}
export interface Cat {
    sightings: CatSighting[];
    name: string;
    colour: CatColour;
    markings: string | undefined;
    collar: string | undefined;
    description: string;
    friendliness: number;
}
