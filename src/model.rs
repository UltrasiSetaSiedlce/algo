#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Box {
    id: usize,
    width: usize,
    height: usize,
}
