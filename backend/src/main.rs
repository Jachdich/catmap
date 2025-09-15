use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tiny_http::{Method, Response, Server};

#[derive(Serialize, Deserialize, Debug)]
struct CatSighting {
    pos: (f64, f64),
    who: String,
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
    description: String,
    best_image: Option<(u32, u32)>,
}

const SCHEMA: &'static str = r#"
CREATE TABLE cats (
    id Integer PRIMARY KEY,
    name Text NOT NULL,
    colour Text NOT NULL,
    markings Text,
    collar Text,
    description Text,
    best_image_a Integer,
    best_image_b Integer
);

CREATE TABLE sightings (
    id Integer PRIMARY KEY,
    lat Real NOT NULL,
    long Real NOT NULL,
    who Text NOT NULL,
    whe BigInt NOT NULL,
    friendliness Integer,
    notes Text,
    for_cat Integer,
    FOREIGN KEY (for_cat) REFERENCES cats(id)
);

CREATE TABLE sighting_images (
    for_sighting Integer,
    url Text NOT NULL,
    FOREIGN KEY (for_sighting) REFERENCES sightings(id)
);
"#;

fn insert_cat(db: &mut Connection, cat: &Cat) -> Result<(), rusqlite::Error> {
    println!("Inserting cat: {:?}", cat);
    let (best_image_a, best_image_b) = match cat.best_image {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    let tx = db.transaction()?;
    let cat_id: u32 = tx.prepare("INSERT INTO cats (name, colour, markings, collar, description, best_image_a, best_image_b) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING id")?
        .query_row(params![cat.name, cat.colour, cat.markings, cat.collar, cat.description, best_image_a, best_image_b], |row| row.get(0))?;

    let mut sighting_smt = 
        tx.prepare("INSERT INTO sightings (lat, long, who, whe, friendliness, notes, for_cat) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING id")?;
    let mut url_smt = tx.prepare("INSERT INTO sighting_images (for_sighting, url) VALUES (?1, ?2)")?;
    for sighting in &cat.sightings {
        let sighting_id: u32 = sighting_smt.query_row(params![sighting.pos.0, sighting.pos.1, sighting.who, sighting.when, sighting.friendliness, sighting.notes, cat_id],
            |row| row.get(0))?;

        for url in &sighting.image_urls {
            url_smt.execute(params![sighting_id, url])?;
        }
    }

    drop(sighting_smt);
    drop(url_smt);

    tx.commit()?;
    Ok(())
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
                (None, _) => None,
                (_, None) => None,
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
/// GET /api/v1/cat/{id}/images/img_id
///

fn handle_api(mut parts: std::str::Split<'_, char>, db: &Connection) -> Result<String, u32> {
    let Some("v1") = parts.next() else {
        return Err(404);
    };
    match parts.next() {
        Some("cat") => {
            let Some(cat_id) = parts.next() else {
                return Err(404);
            };

            let cat_id = cat_id.parse::<u32>().map_err(|_| 404u32)?;

            let result = get_cat(db, cat_id).map_err(|_| 500u32)?;
            let cat = result.ok_or(404u32)?;
            println!("Got cat: {:?}", cat);

            serde_json::to_string(&cat).map_err(|_| 500)
        }
        Some("list_cats") => {
            let cat_list = list_cats(db).map_err(|_| 500u32)?;
            serde_json::to_string(&cat_list).map_err(|_| 500u32)
        }
        Some(_) => Err(404),
        None => Err(404),
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

fn get(url: &str, db: &Connection) -> Result<String, u32> {
    let mut parts = url.split('/');
    parts.next();
    match parts.next() {
        Some("api") => handle_api(parts, db),
        // Some(path) => {
        //     let mut path = vec![path];
        //     path.extend(parts);
        //     handle_file(path)
        // }

        // None => handle_file(vec!["index.html"]),
        _ => Err(404),
    }
}

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    let mut db = rusqlite::Connection::open_in_memory().unwrap();
    db.execute_batch(SCHEMA).unwrap();

    for mut request in server.incoming_requests() {
        println!(
            "received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let response = match request.method() {
            Method::Post => {
                // create something
                let r = request.as_reader();
                let mut body = String::new();
                r.read_to_string(&mut body).unwrap(); // TODO
                let cat: Cat = serde_json::from_str(&body).unwrap();
                insert_cat(&mut db, &cat).unwrap();
                
                Response::from_string("").with_status_code(200)
            }
            Method::Put => {
                // update something

                Response::from_string("").with_status_code(200)
            }
            Method::Get => {
                let url = request.url();
                match get(url, &db) {
                    Ok(result) => Response::from_string(result),
                    Err(code) => Response::from_string("").with_status_code(code),
                }
            }
            _ => Response::from_string("").with_status_code(401),
        };

        request.respond(response).unwrap();
    }
}
