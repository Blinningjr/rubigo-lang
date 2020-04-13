/**
 * Defines Span that keeps location information about a fragment.
 */
pub struct Span<T> {
    pub fragment: T,
    pub line: usize,
    pub offset: usize,
}

