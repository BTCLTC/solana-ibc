use std::{
    borrow::Borrow,
    collections::BTreeMap,
    fmt::{self, Debug},
    sync::Arc,
};

use crate::IbcStore;
use ibc::core::ics26_routing::context::{Module, ModuleId, RouterContext};

#[derive(Default, Clone)]
pub struct Router(pub BTreeMap<ModuleId, Arc<dyn Module>>);

impl Debug for Router {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys = self.0.iter().fold(vec![], |acc, (key, _)| {
            [acc, vec![format!("{}", key)]].concat()
        });

        write!(f, "Router(BTreeMap(key({:?})", keys.join(","))
    }
}

impl ibc::core::ics26_routing::context::Router for Router {
    fn get_route_mut(&mut self, module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module> {
        self.0.get_mut(module_id.borrow()).and_then(Arc::get_mut)
    }

    fn has_route(&self, module_id: &impl Borrow<ModuleId>) -> bool {
        self.0.get(module_id.borrow()).is_some()
    }
}

impl RouterContext for IbcStore {
    type Router = Router;

    fn router(&self) -> &Self::Router {
        todo!()
    }

    fn router_mut(&mut self) -> &mut Self::Router {
        todo!()
    }
}
