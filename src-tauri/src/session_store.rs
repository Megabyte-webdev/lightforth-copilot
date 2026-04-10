use std::sync::{ Arc, Mutex };

#[derive(Clone)]
pub struct SessionStore {
    pub current_id: Arc<Mutex<Option<String>>>, // None = no session
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            current_id: Arc::new(Mutex::new(None)),
        }
    }

    /// Helper – set id
    pub fn set(&self, id: String) {
        *self.current_id.lock().unwrap() = Some(id);
    }

    /// Helper – clear
    pub fn clear(&self) {
        *self.current_id.lock().unwrap() = None;
    }

    /// Helper – get, returning copy
    pub fn get(&self) -> Option<String> {
        self.current_id.lock().unwrap().clone()
    }
}
