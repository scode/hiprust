use std::iter;
use cfg;

pub struct Room {
    id: i64,
    name: String,
}

pub struct RoomHistory;

pub trait RoomClient {
    fn list_rooms(&self) -> iter::Iterator<Room>;
    fn recent_history(&self, &Room) -> RoomHistory;
}

pub trait Client {
    fn rooms(&self) -> RoomClient;
}

pub fn make_client() -> Room {
    Room { id: 5i64, name: "test".to_string() }
}

struct ClientImpl;

