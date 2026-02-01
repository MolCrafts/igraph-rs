use std::thread;

use igraph::*;

// === Foundation tests ===

#[test]
fn test_vector_int_roundtrip() {
    let data = vec![1i64, 2, 3, 4, 5];
    let v = VectorInt::from_slice(&data).unwrap();
    assert_eq!(v.len(), 5);
    assert_eq!(v.to_vec(), data);
}

#[test]
fn test_vector_int_empty() {
    let v = VectorInt::new().unwrap();
    assert!(v.is_empty());
    assert_eq!(v.len(), 0);
}

#[test]
fn test_vector_f64_roundtrip() {
    let data = vec![1.0, 2.5, 3.0];
    let v = Vector::from_slice(&data).unwrap();
    assert_eq!(v.len(), 3);
    assert_eq!(v.to_vec(), data);
}

#[test]
fn test_matrix_dimensions() {
    let m = Matrix::new(3, 4).unwrap();
    assert_eq!(m.nrow(), 3);
    assert_eq!(m.ncol(), 4);
}

// === Graph construction tests ===

#[test]
fn test_empty_graph() {
    let g = Graph::empty(5, false).unwrap();
    assert_eq!(g.vcount(), 5);
    assert_eq!(g.ecount(), 0);
    assert!(!g.is_directed());
}

#[test]
fn test_empty_directed_graph() {
    let g = Graph::empty(3, true).unwrap();
    assert_eq!(g.vcount(), 3);
    assert!(g.is_directed());
}

#[test]
fn test_from_edges() {
    let g = Graph::from_edges(&[(0, 1), (1, 2), (2, 0)], 3, false).unwrap();
    assert_eq!(g.vcount(), 3);
    assert_eq!(g.ecount(), 3);
    assert!(!g.is_directed());
}

#[test]
fn test_from_edges_directed() {
    let g = Graph::from_edges(&[(0, 1), (1, 2)], 3, true).unwrap();
    assert_eq!(g.vcount(), 3);
    assert_eq!(g.ecount(), 2);
    assert!(g.is_directed());
}

// === Graph query tests ===

#[test]
fn test_neighbors() {
    let g = Graph::from_edges(&[(0, 1), (0, 2), (1, 2)], 3, false).unwrap();
    let mut neis = g.neighbors(0, NeighborMode::All).unwrap();
    neis.sort();
    assert_eq!(neis, vec![1, 2]);
}

#[test]
fn test_degree() {
    let g = Graph::from_edges(&[(0, 1), (0, 2), (1, 2)], 3, false).unwrap();
    let deg = g.degree(NeighborMode::All, Loops::Twice).unwrap();
    assert_eq!(deg, vec![2, 2, 2]);
}

#[test]
fn test_edge() {
    let g = Graph::from_edges(&[(0, 1), (2, 3)], 4, false).unwrap();
    let (from, to) = g.edge(0).unwrap();
    assert_eq!(from, 0);
    assert_eq!(to, 1);
    let (from, to) = g.edge(1).unwrap();
    assert_eq!(from, 2);
    assert_eq!(to, 3);
}

// === Graph mutation tests ===

#[test]
fn test_add_vertices() {
    let mut g = Graph::empty(5, false).unwrap();
    g.add_vertices(3).unwrap();
    assert_eq!(g.vcount(), 8);
}

#[test]
fn test_add_edges() {
    let mut g = Graph::empty(4, false).unwrap();
    g.add_edges(&[(0, 1), (2, 3)]).unwrap();
    assert_eq!(g.ecount(), 2);
}

// === Generator tests ===

#[test]
fn test_famous_petersen() {
    let g = Graph::famous("Petersen").unwrap();
    assert_eq!(g.vcount(), 10);
    assert_eq!(g.ecount(), 15);
}

#[test]
fn test_ring() {
    let g = Graph::ring(5, false, false, true).unwrap();
    assert_eq!(g.vcount(), 5);
    assert_eq!(g.ecount(), 5);
}

#[test]
fn test_star() {
    let g = Graph::star(6, StarMode::Undirected, 0).unwrap();
    assert_eq!(g.vcount(), 6);
    assert_eq!(g.ecount(), 5);
}

#[test]
fn test_full() {
    let g = Graph::full(5, false, false).unwrap();
    assert_eq!(g.vcount(), 5);
    assert_eq!(g.ecount(), 10); // C(5,2) = 10
}

#[test]
fn test_kary_tree() {
    let g = Graph::kary_tree(7, 2, TreeMode::Undirected).unwrap();
    assert_eq!(g.vcount(), 7);
    assert_eq!(g.ecount(), 6);
}

#[test]
fn test_erdos_renyi_gnm() {
    let g = Graph::erdos_renyi_gnm(100, 200, false, false).unwrap();
    assert_eq!(g.vcount(), 100);
    assert_eq!(g.ecount(), 200);
}

#[test]
fn test_barabasi() {
    let g = Graph::barabasi(100, 2, false).unwrap();
    assert_eq!(g.vcount(), 100);
    assert!(g.ecount() > 0);
}

// === Path algorithm tests ===

#[test]
fn test_distances() {
    let g = Graph::from_edges(&[(0, 1), (1, 2), (2, 3)], 4, false).unwrap();
    let dist = g.distances(NeighborMode::All).unwrap();
    assert_eq!(dist[0][0], 0.0);
    assert_eq!(dist[0][1], 1.0);
    assert_eq!(dist[0][2], 2.0);
    assert_eq!(dist[0][3], 3.0);
}

#[test]
fn test_diameter() {
    let g = Graph::from_edges(&[(0, 1), (1, 2), (2, 3)], 4, false).unwrap();
    let d = g.diameter(false).unwrap();
    assert_eq!(d, 3.0);
}

// === Centrality tests ===

#[test]
fn test_betweenness() {
    let g = Graph::from_edges(&[(0, 1), (1, 2), (2, 3)], 4, false).unwrap();
    let bt = g.betweenness(false).unwrap();
    assert_eq!(bt.len(), 4);
    // Middle vertices should have higher betweenness
    assert!(bt[1] > bt[0]);
    assert!(bt[2] > bt[3]);
}

#[test]
fn test_closeness() {
    let g = Graph::full(5, false, false).unwrap();
    let cl = g.closeness(NeighborMode::All).unwrap();
    assert_eq!(cl.len(), 5);
    // In a complete graph, all closeness values are equal
    for c in &cl {
        assert!((c - cl[0]).abs() < 1e-10);
    }
}

#[test]
fn test_pagerank() {
    let g = Graph::full(5, false, false).unwrap();
    let pr = g.pagerank(0.85).unwrap();
    assert_eq!(pr.len(), 5);
    let sum: f64 = pr.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6);
}

// === Component tests ===

#[test]
fn test_is_connected() {
    let g = Graph::full(5, false, false).unwrap();
    assert!(g.is_connected(Connectedness::Weak).unwrap());
}

#[test]
fn test_is_disconnected() {
    let g = Graph::empty(5, false).unwrap();
    assert!(!g.is_connected(Connectedness::Weak).unwrap());
}

#[test]
fn test_connected_components() {
    // Two separate triangles
    let g = Graph::from_edges(&[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)], 6, false).unwrap();
    let (membership, csize, no) = g.connected_components(Connectedness::Weak).unwrap();
    assert_eq!(no, 2);
    assert_eq!(membership.len(), 6);
    assert_eq!(csize.len(), 2);
    // Vertices 0,1,2 should be in same component
    assert_eq!(membership[0], membership[1]);
    assert_eq!(membership[1], membership[2]);
    // Vertices 3,4,5 in another
    assert_eq!(membership[3], membership[4]);
    assert_eq!(membership[4], membership[5]);
    assert_ne!(membership[0], membership[3]);
}

// === Community detection tests ===

#[test]
fn test_community_leiden() {
    let g = Graph::famous("Zachary").unwrap();
    let (membership, nb_clusters, _quality) = g.community_leiden(1.0).unwrap();
    assert_eq!(membership.len() as i64, g.vcount());
    assert!(nb_clusters >= 1);
}

#[test]
fn test_community_label_propagation() {
    let g = Graph::famous("Zachary").unwrap();
    let membership = g.community_label_propagation().unwrap();
    assert_eq!(membership.len() as i64, g.vcount());
}

#[test]
fn test_community_fastgreedy() {
    let g = Graph::famous("Zachary").unwrap();
    let (membership, modularity) = g.community_fastgreedy().unwrap();
    assert_eq!(membership.len() as i64, g.vcount());
    assert!(!modularity.is_empty());
}

// === Transformation tests ===

#[test]
fn test_simplify() {
    let mut g = Graph::from_edges(&[(0, 1), (0, 1), (1, 1)], 2, false).unwrap();
    assert_eq!(g.ecount(), 3);
    g.simplify(true, true).unwrap();
    assert_eq!(g.ecount(), 1);
}

#[test]
fn test_to_directed() {
    let mut g = Graph::from_edges(&[(0, 1), (1, 2)], 3, false).unwrap();
    assert!(!g.is_directed());
    g.to_directed(ToDirectedMode::Mutual).unwrap();
    assert!(g.is_directed());
    assert_eq!(g.ecount(), 4); // each undirected edge becomes 2 directed
}

#[test]
fn test_to_undirected() {
    let mut g = Graph::from_edges(&[(0, 1), (1, 0), (1, 2)], 3, true).unwrap();
    assert!(g.is_directed());
    g.to_undirected(ToUndirectedMode::Collapse).unwrap();
    assert!(!g.is_directed());
}

#[test]
fn test_induced_subgraph() {
    let g = Graph::full(5, false, false).unwrap();
    let sub = g.induced_subgraph(&[0, 1, 2]).unwrap();
    assert_eq!(sub.vcount(), 3);
    assert_eq!(sub.ecount(), 3); // C(3,2) = 3
}

// === Full workflow integration test ===

#[test]
fn test_full_workflow() {
    // Create a graph
    let mut g = Graph::empty(3, false).unwrap();

    // Add vertices and edges
    g.add_vertices(2).unwrap();
    g.add_edges(&[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)])
        .unwrap();

    // Query properties
    assert_eq!(g.vcount(), 5);
    assert_eq!(g.ecount(), 5);
    assert!(!g.is_directed());

    // Check connectivity
    assert!(g.is_connected(Connectedness::Weak).unwrap());

    // Compute centrality
    let bt = g.betweenness(false).unwrap();
    assert_eq!(bt.len(), 5);

    // Compute PageRank
    let pr = g.pagerank(0.85).unwrap();
    let sum: f64 = pr.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6);

    // Get distances
    let dist = g.distances(NeighborMode::All).unwrap();
    assert_eq!(dist.len(), 5);

    // Compute diameter
    let d = g.diameter(false).unwrap();
    assert!(d > 0.0);

    // Create induced subgraph
    let sub = g.induced_subgraph(&[0, 1, 2]).unwrap();
    assert_eq!(sub.vcount(), 3);
}

// === Thread safety tests ===

#[test]
fn test_graph_send_to_thread() {
    let g = Graph::from_edges(&[(0, 1), (1, 2), (2, 0)], 3, false).unwrap();

    let handle = thread::spawn(move || {
        assert_eq!(g.vcount(), 3);
        assert_eq!(g.ecount(), 3);
        assert!(g.is_connected(Connectedness::Weak).unwrap());
        g.diameter(false).unwrap()
    });

    let diameter = handle.join().unwrap();
    assert_eq!(diameter, 1.0);
}

#[test]
fn test_multiple_graphs_in_threads() {
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                let n = (i + 3) as i64;
                let g = Graph::full(n, false, false).unwrap();
                assert_eq!(g.vcount(), n);
                g.ecount()
            })
        })
        .collect();

    for (i, h) in handles.into_iter().enumerate() {
        let n = (i + 3) as i64;
        let expected = n * (n - 1) / 2;
        assert_eq!(h.join().unwrap(), expected);
    }
}
