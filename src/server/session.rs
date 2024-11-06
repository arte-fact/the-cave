use std::time;

pub struct Session {
    pub id: u128,
    pub last_update: u128,
    pub expiration: u128,
}

impl Session {
    pub fn new(expiration: u128) -> Self {
        let time = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        Self {
            id: time,
            last_update: time,
            expiration,
        }
    }

    pub fn update(&mut self) {
        self.last_update = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
    }

    pub fn is_expired(&self) -> bool {
        self.last_update + self.expiration < time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
