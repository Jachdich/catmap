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
    best_image_b Integer
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
    mime Text NOT NULL
)
"#;

fn insert_sightings(db: &Connection, sightings: &[CatSighting], cat_id: u32) -> Result<(), rusqlite::Error> {
    let mut sighting_smt = 
        db.prepare("INSERT INTO sightings (lat, long, who, whe, friendliness, notes, for_cat) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING id")?;
    let mut url_smt = db.prepare("INSERT INTO sighting_images (for_sighting, url) VALUES (?1, ?2)")?;
    for sighting in sightings {
        let sighting_id: u32 = sighting_smt.query_row(params![sighting.pos.0, sighting.pos.1, sighting.who, sighting.when, sighting.friendliness, sighting.notes, cat_id],
            |row| row.get(0))?;

        for url in &sighting.image_urls {
            url_smt.execute(params![sighting_id, url])?;
        }
    }

    Ok(())
}

fn insert_cat(db: &mut Connection, cat: &Cat) -> Result<(), rusqlite::Error> {
    println!("Inserting cat: {:?}", cat);
    let (best_image_a, best_image_b) = match cat.best_image {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    let tx = db.transaction()?;
    let cat_id: u32 = tx.prepare("INSERT INTO cats (name, colour, markings, collar, description, best_image_a, best_image_b) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING id")?
        .query_row(params![cat.name, cat.colour, cat.markings, cat.collar, cat.description, best_image_a, best_image_b], |row| row.get(0))?;

    insert_sightings(&tx, &cat.sightings, cat_id)?;

    tx.commit()?;
    Ok(())
}

fn insert_image(db: &mut Connection, image: &[u8], mime: &Mime) -> Result<u32, rusqlite::Error> {
    db.prepare("INSERT INTO images (data, mime) VALUES (?1, ?2) RETURNING id")?.query_row(params![image, mime.essence_str()], |row| row.get(0))
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
/// GET /api/v1/cat/{id}/
/// GET /api/v1/image/img_id.extension
/// POST /api/v1/cat
/// POST /api/v1/image

fn handle_api(path: &[&str], db: &Connection) -> Result<Response<std::io::Cursor<Vec<u8>>>, u32> {
    match path {
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

// fn handle_file<T>(path: Vec<&str>) -> Result<Response<T>, u32> {
//     // primative defence against attack. Don't do this in production
//     if path.contains(&"..") {
//         return Err(404);
//     }
//     if path.first().is_some_and(|s| *s == "") {
//         return Err(404);
//     }

//     let path = "static/".to_owned() + &path.join("/");
//     let file = std::fs::File::open(std::path::Path::new(&path));
//     let text = std::fs::read_to_string(file).map_err(|_| 404u32)?;
//     Ok(Response::from_file(file))
// }

fn get(url: &str, db: &Connection) -> Result<Response<std::io::Cursor<Vec<u8>>>, u32> {
    let mut parts = url.split('/');
    parts.next();
    let path: Vec<_> = parts.collect();
    match path.as_slice() {
        ["api", rest @ ..] => handle_api(rest, db),
        // Some(path) => {
        //     let mut path = vec![path];
        //     path.extend(parts);
        //     handle_file(path)
        // }

        // None => handle_file(vec!["index.html"]),
        _ => Err(404),
    }
}

fn post(request: &mut Request, db: &mut Connection) -> Result<Response<std::io::Cursor<Vec<u8>>>, u32> {
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
            insert_sightings(db, &[sighting], cat_id).unwrap();
            Ok(Response::from_string(""))
        },
        ["api", "v1", "cat"] => {
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).unwrap();
            let cat: Cat = serde_json::from_str(&body).unwrap();
            insert_cat(db, &cat).unwrap();
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
            // if request.body_length().is_none_or(|l| l > 1 * 1024 * 1024 * 1024) {
            //     return Err(400);
            // }

            let r = request.as_reader();
            let mut body = Vec::new();
            r.read_to_end(&mut body).map_err(|_| 500u32)?;
            let f_number = insert_image(db, &body, &mime).map_err(|_| 500u32)?;
            // TODO finish this, make it not guessable...
            // figure out how to make this website not literally a free file server
            Ok(Response::from_string(format!("/api/v1/image/{}.{}", f_number, mime.subtype())))
        },
        _ => Err(404),
    }
}

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    // let mut db = rusqlite::Connection::open_in_memory().unwrap();
    let mut db = rusqlite::Connection::open("catmap.db").unwrap();
    db.execute_batch(SCHEMA).unwrap();

    for mut request in server.incoming_requests() {
        println!(
            "received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            // request.headers()
            ""
        );

        let response = match request.method() {
            Method::Post => {
                // create something
                match post(&mut request, &mut db) {
                    Ok(result) => result,
                    Err(code) => Response::from_string("").with_status_code(code),
                }
            }

            Method::Put => {
                // update something

                Response::from_string("").with_status_code(200)
            }
            Method::Get => {
                let url = request.url();
                match get(url, &db) {
                    Ok(result) => result,
                    Err(code) => Response::from_string("").with_status_code(code),
                }
            }
            _ => Response::from_string("").with_status_code(401),
        };

        request.respond(response).unwrap();
    }
}
