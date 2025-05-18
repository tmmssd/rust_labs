pub struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {name: name.to_string(), size: 0, count: 0}
    }

    pub fn size(self, n: u32) -> Node {
        Node { size: n, ..self}
    }

    pub fn count(self, c: u32) -> Node {
        Node {count: c, ..self}
    }

    pub fn to_string(&self) -> String {
        format!("name:{} size:{} count:{}", self.name, self.size, self.count)
    }

    pub fn grow(&mut self){
        self.size += 1;
    }

    pub fn inc(&mut self){
        self.count += 1;
    }
}


fn main() {
    let mut node: Node = Node::new("nodo").size(10).count(5);
    node.grow();
    println!("{}", node.to_string());
}
