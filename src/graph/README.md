# Graph

## Phase 0: Data Structure Warm-up

- [ ] Vec, HashMap, HashSet - basic operations
- [ ] BinaryHeap (priority queue) - min-heap pattern (`Reverse`)
- [ ] VecDeque (queue)
- [ ] Index-based graph representation: `Vec<Vec<(usize, u32)>>`

## Phase 1: Graph Traversal Basics

- [ ] Adjacency matrix vs adjacency list - implement and compare
- [ ] DFS - recursive version
- [ ] DFS - stack-based (iterative)
- [ ] BFS - VecDeque-based
- [ ] Directed vs undirected graphs
- [ ] Cycle detection (DFS-based)

## Phase 2: Shortest Path Algorithms

- [ ] Dijkstra - BinaryHeap-based
- [ ] A* - heuristic function design (Manhattan, Euclidean)
- [ ] Bellman-Ford - negative weights / negative cycle detection

---

## Phase 3+: Graph Algorithm Catalog

### Topological Sort [Ordering]
- Prereqs: DFS or BFS
- DAG dependency ordering. Build systems, task scheduling.

### Union-Find / Disjoint Set [Connectivity]
- Prereqs: none beyond basics
- "Are these two nodes connected?" in near O(1). Path compression, union by rank.

### Bipartite Check [Connectivity]
- Prereqs: BFS or DFS
- 2-colorability test. Simple extension of graph traversal.

### MST - Prim / Kruskal [Minimum Spanning Tree]
- Prereqs: BinaryHeap (Prim), Union-Find (Kruskal)
- Minimum cost to connect all nodes. Network design problems.

### SCC - Tarjan / Kosaraju [Connectivity]
- Prereqs: DFS, directed graphs
- Strongly connected components in directed graphs. Conceptually tricky despite simple code.

### Floyd-Warshall [All-Pairs Shortest Path]
- Prereqs: DP (dynamic programming) concepts
- All-pairs shortest path via 3 nested loops. Small/dense graphs only.

### Bidirectional Search [Search Optimization]
- Prereqs: BFS/Dijkstra + experience hitting performance limits
- Meet-in-the-middle strategy. Worth trying after you feel Dijkstra being slow.

### IDA* [Search Optimization]
- Prereqs: A*, iterative deepening concept
- Memory-efficient A* variant. Useful for very large search spaces.

### Eulerian Path / Circuit [Path & Cycle]
- Prereqs: DFS, degree counting
- "Visit every edge exactly once." Implementation is moderate, practical use is rare.

### Johnson's Algorithm [All-Pairs Shortest Path]
- Prereqs: Bellman-Ford + Dijkstra
- Reweighting trick to handle negative edges, then Dijkstra on all pairs. Sparse graph alternative to Floyd-Warshall.

### Network Flow - Ford-Fulkerson / Edmonds-Karp [Flow]
- Prereqs: BFS, new problem domain (capacity/residual graphs)
- Max-flow / min-cut. Entirely different way of thinking about graphs.

### Bipartite Matching [Matching]
- Prereqs: Network Flow or augmenting path concept
- Optimal assignment problems. Builds on flow ideas.

### Graph Coloring [Coloring]
- Prereqs: backtracking, constraint satisfaction
- NP-hard. Greedy heuristics exist but no efficient exact solution.

### Hamiltonian Path [Path & Cycle]
- Prereqs: backtracking, bitmask DP
- "Visit every node exactly once." NP-complete. Brute force or DP on subsets.

