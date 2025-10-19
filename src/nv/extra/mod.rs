use crate::nv::primitive::{ShadedVertex, VertexColor};

pub struct Triangle<T> {
    pub vertices: [ShadedVertex<T>; 3],
}
