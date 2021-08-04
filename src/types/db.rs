use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

use super::handler_types::Todo;

pub type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;
