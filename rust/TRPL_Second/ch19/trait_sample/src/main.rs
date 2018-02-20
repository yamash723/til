trait GGraph<Node, Edge> { }
trait AGraph {
    type Node;
    type Edge;
}

struct GenericsExample { }
impl GGraph<u32, String> for GenericsExample { }

struct AssociatedExample { }

impl AGraph for AssociatedExample {
    type Node = u32;
    type Edge = String;
}

fn distance1<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &E) -> u32 {
    unimplemented!()
}

fn distance2<G: AGraph>(graph: &G, start: &G::Node, end: &G::Edge) -> u32 {
    unimplemented!()
}

fn distance3<N, E>(graph: &GGraph<N, E>, start: &N, end: &E) -> u32 {
    unimplemented!()
}

fn main() {
    distance1(&GenericsExample { }, &1, &String::from("end"));
    distance2(&AssociatedExample { }, &1, &String::from("end"));
    distance3(&GenericsExample { }, &1, &String::from("end"));
}
