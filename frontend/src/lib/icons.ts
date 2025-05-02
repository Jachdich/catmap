import { Icon } from "leaflet";

export class CatIcon extends Icon {
    constructor(url: string) {
        const scl = 10;
        super({
            iconUrl: url,
            iconSize:     [299 / scl, 299 / scl],
            iconAnchor:   [153 / scl, 182 / scl],
        });
    }

}
export const cat_icon = new CatIcon("/catmap/catmeow.png");
export const cat_icon_sel = new CatIcon("/catmap/catmeow_highlight.png");
