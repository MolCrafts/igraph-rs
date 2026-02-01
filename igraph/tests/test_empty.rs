use igraph::*;

#[test]
fn empty_directed_zero_vertices() {
    let g = Graph::empty(0, true).unwrap();
    assert_eq!(g.vcount(), 0);
    assert_eq!(g.ecount(), 0);
    assert!(g.is_directed());
}

#[test]
fn empty_undirected_zero_vertices() {
    let g = Graph::empty(0, false).unwrap();
    assert_eq!(g.vcount(), 0);
    assert_eq!(g.ecount(), 0);
    assert!(!g.is_directed());
}

#[test]
fn empty_directed_20_vertices() {
    let g = Graph::empty(20, true).unwrap();
    assert_eq!(g.vcount(), 20);
    assert_eq!(g.ecount(), 0);
    assert!(g.is_directed());
}

#[test]
fn empty_undirected_30_vertices() {
    let g = Graph::empty(30, false).unwrap();
    assert_eq!(g.vcount(), 30);
    assert_eq!(g.ecount(), 0);
    assert!(!g.is_directed());
}
