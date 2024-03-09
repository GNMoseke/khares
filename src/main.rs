#[macro_use] extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

// TODO: move these to their own file/handler crate
#[get("/")]
fn list_backlog_entries() {
    unimplemented!()
    // Return all
    // TODO: allow pagination, sorting, type args
}

#[post("/item")]
fn create_backlog_entry() {
    unimplemented!()
    /*
    Request content needs:
        - Item type enum
        - Title
        - additional metadata per type
     */

    // TODO: store by unique ID
}

#[get("/item?<id>")]
fn get_backlog_entry(id: &str) {
    unimplemented!()
    /* 
    Request content needs:
        - unique ID of backlog entry
    */
}

#[launch]
fn rocket() -> _ {
    // Mount paths by namespace
    rocket::build()
        .mount("/util", routes![ping])
        .mount("/backlog", routes![list_backlog_entries, create_backlog_entry, get_backlog_entry])
}
