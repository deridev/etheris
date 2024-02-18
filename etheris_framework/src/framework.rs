use std::{collections::HashMap, sync::Arc};

use crate::{Command, EtherisClient};

pub struct Framework {
    pub client: Arc<EtherisClient>,
    pub commands: HashMap<String, Box<(dyn Command + Send + Sync)>>,
}
