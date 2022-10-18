use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;
use std::str::FromStr;

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

trait Problem<T>: Sized {
    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<T>;
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let problem = Toy::from_reader(stdin)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct Toy {
    parts: Graph,
}

type VertexId = usize;

struct Graph {
    vertices: HashMap<VertexId, Vertex>,
}

struct Vertex {
    neighbours: HashSet<VertexId>,
    value: u32,
}

impl Problem<u32> for Toy {
    fn from_reader(reader: impl BufRead) -> Result<Self> {
        fn split_parse_iter<T>(line: &str) -> impl Iterator<Item = Result<T, T::Err>> + '_
        where
            T: FromStr,
            T::Err: std::error::Error,
        {
            line.split_whitespace().map(|entry| entry.parse::<T>())
        }

        let mut lines = reader.lines();

        let first_line = lines.next().ok_or("Missing first line")??;
        let mut first_line = split_parse_iter(&first_line);
        let vertex_count = first_line.next().ok_or("Missing vertex count")??;
        let edge_count = first_line.next().ok_or("Missing edge capacity")??;

        debug_assert!((1..=1_000).contains(&vertex_count));
        debug_assert!((0..=2_000).contains(&edge_count));
        debug_assert!(first_line.next().is_none());

        let second_line = lines.next().ok_or("Missing second line")??;
        let second_line = split_parse_iter(&second_line);
        let vertices = second_line
            .map(|value| value.map(Vertex::new))
            .enumerate()
            .map(|(id, vertex)| vertex.map(|vertex| (id, vertex)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        debug_assert_eq!(vertices.len(), vertex_count);
        for vertex in &vertices {
            debug_assert!((0..=100_000).contains(&vertex.1.value))
        }

        let parts = Graph { vertices };
        let mut toy = Toy { parts };

        let mut line_counter = 0;
        for line in lines {
            let line = line?;
            let mut edge = split_parse_iter(&line);
            let first_vertex_id = edge.next().ok_or("Missing first vertex")??;
            let second_vertex_id = edge.next().ok_or("Missing second vertex")??;

            debug_assert!((1..=vertex_count).contains(&first_vertex_id));
            debug_assert!((1..=vertex_count).contains(&second_vertex_id));
            debug_assert_ne!(first_vertex_id, second_vertex_id);
            debug_assert!(edge.next().is_none());

            toy.parts
                .insert_edge(first_vertex_id - 1, second_vertex_id - 1);

            line_counter += 1;
        }
        debug_assert_eq!(line_counter, edge_count);

        toy.parts.vertices.shrink_to_fit();
        for vertex in &mut toy.parts.vertices.values_mut() {
            vertex.neighbours.shrink_to_fit()
        }

        Ok(toy)
    }

    fn solve(mut self) -> Result<u32> {
        let mut cost = 0;
        let mut vertices = (0..self.parts.vertices.len()).collect::<Vec<_>>();
        vertices.sort_unstable_by_key(|vertex_id| self.parts.vertices[vertex_id].value);
        while let Some(vertex_id) = vertices.pop() {
            let vertex = self.parts.vertices.remove(&vertex_id).ok_or("Missing vertex")?;
            let mut neighbours_value_sum = 0;
            for neighbour_id in vertex.neighbours {
                let neighbour = self.parts.vertices.get_mut(&neighbour_id).ok_or("Missing vertex")?;
                neighbour.neighbours.remove(&vertex_id);
                neighbours_value_sum += neighbour.value;
            }
            cost += neighbours_value_sum;
        }
        Ok(cost)
    }
}

impl Graph {
    fn insert_edge(&mut self, first_vertex_id: VertexId, second_vertex_id: VertexId) {
        fn insert_neighbour(
            graph: &mut Graph,
            first_vertex_id: VertexId,
            second_vertex_id: VertexId,
        ) {
            if let Some(vertex) = graph.vertices.get_mut(&first_vertex_id) {
                vertex.neighbours.insert(second_vertex_id);
            } else {
                debug_assert!(false);
            }
        }

        insert_neighbour(self, first_vertex_id, second_vertex_id);
        insert_neighbour(self, second_vertex_id, first_vertex_id);
    }
}

impl Vertex {
    fn new(value: u32) -> Self {
        Self {
            neighbours: HashSet::new(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Toy {
            parts: Graph {
                vertices: HashMap::from([
                    (
                        0,
                        Vertex {
                            neighbours: HashSet::from([3, 1]),
                            value: 10,
                        },
                    ),
                    (
                        1,
                        Vertex {
                            neighbours: HashSet::from([0, 2]),
                            value: 20,
                        },
                    ),
                    (
                        2,
                        Vertex {
                            neighbours: HashSet::from([1]),
                            value: 30,
                        },
                    ),
                    (
                        3,
                        Vertex {
                            neighbours: HashSet::from([0]),
                            value: 40,
                        },
                    ),
                ]),
            },
        };
        let actual = problem.solve()?;
        assert_eq!(40, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Toy {
            parts: Graph {
                vertices: HashMap::from([
                    (
                        0,
                        Vertex {
                            neighbours: HashSet::from([1]),
                            value: 100,
                        },
                    ),
                    (
                        1,
                        Vertex {
                            neighbours: HashSet::from([0, 2, 3]),
                            value: 100,
                        },
                    ),
                    (
                        2,
                        Vertex {
                            neighbours: HashSet::from([3, 1]),
                            value: 100,
                        },
                    ),
                    (
                        3,
                        Vertex {
                            neighbours: HashSet::from([2, 1]),
                            value: 100,
                        },
                    ),
                ]),
            },
        };
        let actual = problem.solve()?;
        assert_eq!(400, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = Toy {
            parts: Graph {
                vertices: HashMap::from([
                    (
                        0,
                        Vertex {
                            neighbours: HashSet::from([4, 5, 2, 3]),
                            value: 40,
                        },
                    ),
                    (
                        1,
                        Vertex {
                            neighbours: HashSet::from([4]),
                            value: 10,
                        },
                    ),
                    (
                        2,
                        Vertex {
                            neighbours: HashSet::from([0, 3]),
                            value: 20,
                        },
                    ),
                    (
                        3,
                        Vertex {
                            neighbours: HashSet::from([6, 4, 5, 2, 0]),
                            value: 10,
                        },
                    ),
                    (
                        4,
                        Vertex {
                            neighbours: HashSet::from([0, 3, 1, 6]),
                            value: 20,
                        },
                    ),
                    (
                        5,
                        Vertex {
                            neighbours: HashSet::from([3, 0]),
                            value: 80,
                        },
                    ),
                    (
                        6,
                        Vertex {
                            neighbours: HashSet::from([3, 4]),
                            value: 40,
                        },
                    ),
                ]),
            },
        };
        let actual = problem.solve()?;
        assert_eq!(160, actual);
        Ok(())
    }
}
