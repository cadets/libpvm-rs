use std::sync::mpsc::SyncSender;

use super::persist::DBTr;
use super::Node;

use std::collections::HashMap;
use packstream::values::Value;

pub struct DB {
    persist_pipe: SyncSender<DBTr>,
}

impl DB {
    pub fn create(pipe: SyncSender<DBTr>) -> DB {
        DB { persist_pipe: pipe }
    }

    pub fn create_node(&mut self, node: &Node) {
        let mut props: HashMap<&'static str, Value> = HashMap::new();
        props.insert("db_id", node.db_id.into());
        props.insert("uuid", node.uuid.into());
        props.insert("pid", node.pid.into());
        props.insert("cmdline", node.cmdline.clone().into());
        self.persist_pipe
            .send(DBTr::CreateNode(vec!["Node", "Process"], props.into()))
            .expect("Database worker closed queue unexpectadly")
    }

    pub fn create_rel(&mut self, src: &Node, dst: &Node, class: String) {
        let mut props: HashMap<&'static str, Value> = HashMap::new();
        props.insert("src", src.db_id.into());
        props.insert("dst", dst.db_id.into());
        props.insert("class", class.into());
        self.persist_pipe
            .send(DBTr::CreateRel(props.into()))
            .expect("Database worker closed queue unexpectadly")
    }

    pub fn update_node(&mut self, node: &Node) {
        let mut props: HashMap<&'static str, Value> = HashMap::new();
        props.insert("db_id", node.db_id.into());
        props.insert("pid", node.pid.into());
        props.insert("cmdline", node.cmdline.clone().into());
        self.persist_pipe
            .send(DBTr::UpdateNode(props.into()))
            .expect("Database worker closed queue unexpectadly")
    }
}
