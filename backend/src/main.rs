use std::{io::Read, str::FromStr};

use mime::Mime;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tiny_http::{Header, HeaderField, Method, Request, Response, Server};

#[derive(Serialize, Deserialize, Debug)]
struct CatSighting {
    pos: (f64, f64),
    who: Option<String>,
    when: u64,
    friendliness: Option<u8>,
    notes: Option<String>,
    image_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cat {
    id: u32,
    sightings: Vec<CatSighting>,
    name: String,
    colour: String,
    markings: Option<String>,
    collar: Option<String>,
    description: Option<String>,
    best_image: Option<(u32, u32)>,
}

const SCHEMA: &'static str = r#"
CREATE TABLE IF NOT EXISTS cats (
    id Integer PRIMARY KEY,
    name Text NOT NULL,
    colour Text NOT NULL,
    markings Text,
    collar Text,
    description Text,
    best_image_a Integer,
    best_image_b Integer,

    created_time Integer NOT NULL,
    created_ip Integer NOT NULL
);

CREATE TABLE IF NOT EXISTS sightings (
    id Integer PRIMARY KEY,
    lat Real NOT NULL,
    long Real NOT NULL,
    who Text,
    whe BigInt NOT NULL,
    friendliness Integer,
    notes Text,
    for_cat Integer,

    created_time Integer NOT NULL,
    created_ip Integer NOT NULL,
    FOREIGN KEY (for_cat) REFERENCES cats(id)
);

CREATE TABLE IF NOT EXISTS sighting_images (
    for_sighting Integer,
    url Text NOT NULL,
    FOREIGN KEY (for_sighting) REFERENCES sightings(id)
);

CREATE TABLE IF NOT EXISTS images (
    id Integer PRIMARY KEY,
    data BLOB NOT NULL,
    mime Text NOT NULL,
    
    created_time Integer NOT NULL,
    created_ip Integer NOT NULL
);

CREATE TABLE IF NOT EXISTS cat_edits (
    for_cat Integer NOT NULL,
    edit_time Integer NOT NULL,
    edit_ip Text NOT NULL,

    colour Text NOT NULL,
    markings Text,
    collar Text,
    description Text,
    best_image_a Integer,
    best_image_b Integer,

    FOREIGN KEY (for_cat) REFERENCES cats(id)
);
"#;

fn insert_sightings(db: &Connection, sightings: &[CatSighting], cat_id: u32, ip: &str) -> Result<(), rusqlite::Error> {
    let mut sighting_smt = 
        db.prepare("INSERT INTO sightings (lat, long, who, whe, friendliness, notes, for_cat, created_time, created_ip) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, cast((julianday('now') - 2440587.5) * 86400 * 1000 as integer), ?8) RETURNING id")?;
    let mut url_smt = db.prepare("INSERT INTO sighting_images (for_sighting, url) VALUES (?1, ?2)")?;
    for sighting in sightings {
        let sighting_id: u32 = sighting_smt.query_row(params![sighting.pos.0, sighting.pos.1, sighting.who, sighting.when, sighting.friendliness, sighting.notes, cat_id, ip],
            |row| row.get(0))?;

        for url in &sighting.image_urls {
            url_smt.execute(params![sighting_id, url])?;
        }
    }

    Ok(())
}

fn insert_cat(db: &mut Connection, cat: &Cat, ip: &str) -> Result<(), rusqlite::Error> {
    let (best_image_a, best_image_b) = match cat.best_image {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    let tx = db.transaction()?;
    let cat_id: u32 = tx.prepare("INSERT INTO cats (name, colour, markings, collar, description, best_image_a, best_image_b, created_time, created_ip) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, cast((julianday('now') - 2440587.5) * 86400 * 1000 as integer), ?8) RETURNING id")?
        .query_row(params![cat.name, cat.colour, cat.markings, cat.collar, cat.description, best_image_a, best_image_b, ip], |row| row.get(0))?;

    insert_sightings(&tx, &cat.sightings, cat_id, ip)?;

    tx.commit()?;
    Ok(())
}

/// NOTE!!! does not update the name!!!
fn update_cat(db: &mut Connection, id: u32, cat: &Cat, ip: &str) -> Result<(), rusqlite::Error> {
    let (best_image_a, best_image_b) = match cat.best_image {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    let old_cat = get_cat(db, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)?;
    let (old_best_image_a, old_best_image_b) = match old_cat.best_image {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    let tx = db.transaction()?;
    tx.prepare("UPDATE cats SET colour = ?1, markings = ?2, collar = ?3, description = ?4, best_image_a = ?5, best_image_b = ?6 WHERE id = ?7")?
        .execute(params![cat.colour, cat.markings, cat.collar, cat.description, best_image_a, best_image_b, id])?;

    tx.prepare("INSERT INTO cat_edits VALUES (?1, cast((julianday('now') - 2440587.5) * 86400 * 1000 as integer), ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?.execute(params![
        id, ip, old_cat.colour, old_cat.markings, old_cat.collar, old_cat.description, old_best_image_a, old_best_image_b
    ])?;
    
    tx.commit()?;
    Ok(())
}

fn insert_image(db: &mut Connection, image: &[u8], mime: &Mime, ip: &str) -> Result<u32, rusqlite::Error> {
    db.prepare("INSERT INTO images (data, mime, created_time, created_ip) VALUES (?1, ?2, cast((julianday('now') - 2440587.5) * 86400 * 1000 as integer), ?3) RETURNING id")?.query_row(params![image, mime.essence_str(), ip], |row| row.get(0))
}

fn get_image(db: &Connection, id: u32) -> Result<(Vec<u8>, Mime), rusqlite::Error> {
    db.prepare("SELECT data, mime FROM images WHERE id = ?1")?.query_row([id], |row| {
        let mime_str: String = row.get(1)?;
        Ok((row.get(0)?, mime_str.parse::<Mime>().unwrap()))
    })
}

fn get_images_for_sighting(db: &Connection, id: u32) -> Result<Vec<String>, rusqlite::Error> {
    db.prepare("SELECT * FROM sighting_images WHERE for_sighting = ?1")?
        .query_map([id], |row| row.get(1))?
        .collect()
}
fn get_sightings_for_cat(db: &Connection, id: u32) -> Result<Vec<CatSighting>, rusqlite::Error> {
    db.prepare("SELECT * FROM sightings WHERE for_cat = ?1")?
        .query_map([id], |row| {
            let lat = row.get(1)?;
            let long = row.get(2)?;
            Ok(CatSighting {
                pos: (lat, long),
                who: row.get(3)?,
                when: row.get(4)?,
                friendliness: row.get(5)?,
                notes: row.get(6)?,
                image_urls: get_images_for_sighting(db, row.get(0)?)?,
            })
        })?
        .collect()
}
fn get_cat(db: &Connection, id: u32) -> Result<Option<Cat>, rusqlite::Error> {
    db.prepare("SELECT * FROM cats WHERE id = ?1")?
        .query_row([id], |row| {
            let image_a: Option<u32> = row.get(6)?;
            let image_b: Option<u32> = row.get(7)?;
            let best_image = match (image_a, image_b) {
                (Option::None, _) => None,
                (_, Option::None) => None,
                (Some(a), Some(b)) => Some((a, b)),
            };
            Ok(Cat {
                id,
                sightings: get_sightings_for_cat(db, id)?,
                name: row.get(1)?,
                colour: row.get(2)?,
                markings: row.get(3)?,
                collar: row.get(4)?,
                description: row.get(5)?,
                best_image,
            })
        })
        .optional()
}


fn list_cats(db: &Connection) -> Result<Vec<u32>, rusqlite::Error> {
    db.prepare("SELECT id FROM cats")?
        .query_map([], |row| row.get(0))?
        .collect()
}

///
/// GET /api/v1/cat/{id}
/// GET /api/v1/image/img_id.extension
/// POST /api/v1/cat
/// POST /api/v1/image
/// PUT /api/v1/cat/{id}

fn get(url: &str, db: &Connection) -> Result<Response<std::io::Cursor<Vec<u8>>>, u32> {
    let mut parts = url.split('/');
    parts.next();
    parts.next();
    let path: Vec<_> = parts.collect();
    match path.as_slice() {
        ["v1", "cat", cat_id] => {
            let cat_id = cat_id.parse::<u32>().map_err(|_| 404u32)?;

            let result = get_cat(db, cat_id).map_err(|_| 500u32)?;
            let cat = result.ok_or(404u32)?;

            let data = serde_json::to_string(&cat).map_err(|_| 500u32)?;
            Ok(Response::from_string(data))
        }
        ["v1", "list_cats"] => {
            let cat_list = list_cats(db).map_err(|_| 500u32)?;
            let data = serde_json::to_string(&cat_list).map_err(|_| 500u32)?;
            Ok(Response::from_string(data))
        }
        ["v1", "image", image_name] => {
            let Some((image_id, image_format)) = image_name.split_once('.') else {
                println!("No split");
                return Err(404);
            };

            let image_id = image_id.parse::<u32>().map_err(|e| {println!("{:?}", e); 404u32 })?;
            let (image_data, mime) = get_image(db, image_id).map_err(|e| {println!("{:?}", e); 404u32 })?; // TODO....
            println!("{}, {}", image_format, mime);
            if image_format != mime.subtype() {
                return Err(404);
            }
            Ok(Response::from_data(image_data).with_header(
                format!("Content-Type: {}", mime.essence_str())
                .parse::<Header>()
                .unwrap(),
            ))
        }
        _ => Err(404),
    }
}

fn post(request: &mut Request, ip: &str, db: &mut Connection) -> Result<Response<std::io::Cursor<Vec<u8>>>, u32> {
    let url = request.url();
    let mut parts = url.split("/");
    parts.next();
    let path: Vec<_> = parts.collect();
    match path.as_slice() {
        ["api", "v1", "cat", cat_id, "sightings"] => {
            let cat_id = cat_id.parse::<u32>().map_err(|_| 404u32)?;
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).unwrap();
            let sighting: CatSighting = serde_json::from_str(&body).unwrap();
            insert_sightings(db, &[sighting], cat_id, ip).unwrap();
            Ok(Response::from_string(""))
        },
        ["api", "v1", "cat"] => {
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).unwrap();
            let cat: Cat = serde_json::from_str(&body).unwrap();
            insert_cat(db, &cat, ip).unwrap();
            Ok(Response::from_string(""))
        },
        ["api", "v1", "image"] => {
            let mime = request
                .headers()
                .iter()
                .find(|header| header.field == HeaderField::from_str("Content-Type").unwrap())
                .ok_or(400u32)?
                .value
                .as_str()
                .parse::<mime::Mime>()
                .map_err(|_| 400u32)?;
            if mime.type_() != mime::IMAGE {
                return Err(400);
            }
            // Limit to max of 1 megabyte
            // for now, just deny the request if it doesn't have a length. idk why this would happen
            if request.body_length().is_none_or(|l| l > 1 * 1024 * 1024 * 1024) {
                return Err(400);
            }

            let r = request.as_reader();
            let mut body = Vec::new();
            r.read_to_end(&mut body).map_err(|_| 500u32)?;
            let f_number = insert_image(db, &body, &mime, ip).map_err(|_| 500u32)?;
            // TODO finish this, make it not guessable...
            // figure out how to make this website not literally a free file server
            Ok(Response::from_string(format!("/api/v1/image/{}.{}", f_number, mime.subtype())))
        },
        _ => Err(404),
    }
}

fn put(request: &mut Request, ip: &str, db: &mut Connection) -> Result<Response<std::io::Cursor<Vec<u8>>>, u32> {
    let url = request.url();
    let mut parts = url.split("/");
    parts.next();
    let path: Vec<_> = parts.collect();
    match path.as_slice() {
        ["api", "v1", "cat", cat_id] => {
            let cat_id = cat_id.parse::<u32>().map_err(|_| 404u32)?;
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).unwrap();
            let cat: Cat = serde_json::from_str(&body).unwrap();
            update_cat(db, cat_id, &cat, ip).unwrap();
            Ok(Response::from_string(""))
        },
        _ => Err(404),
    }
}

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    let mut db = rusqlite::Connection::open("catmap.db").unwrap();
    db.execute_batch(SCHEMA).unwrap();

    for mut request in server.incoming_requests() {
        println!(
            "received request! method: {:?}, url: {:?}",
            request.method(),
            request.url(),
        );
        let ip = match request
            .headers()
            .iter()
            .find(|header| header.field == HeaderField::from_str("X-Forwarded-For").unwrap()) {
            Some(header) => header.value.to_string(),
            None => request.remote_addr().unwrap().ip().to_string(),
        };

        let response = match request.method() {
            Method::Post => post(&mut request, &ip, &mut db),
            Method::Put => put(&mut request, &ip, &mut db),
            Method::Get => get(request.url(), &db),
            _ => Err(401),
        };

        let response = match response {
            Ok(result) => result,
            Err(code) => Response::from_string("").with_status_code(code),
        };

        request.respond(response).unwrap();
    }
}
