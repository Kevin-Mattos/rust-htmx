pub struct RaffleRepository {
}


impl RaffleRepository {
    pub fn get_all(&self) ->[Raffle;2]  {
        [Raffle {id: 1, name: "um".to_string()}, Raffle {id: 2, name: "dois".to_string()}]
    }
}


#[derive(serde::Serialize)]
pub struct Raffle {
    pub id: i32,
    pub name: String,
}