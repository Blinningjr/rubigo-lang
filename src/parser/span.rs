/**
 * Defines Span that keeps location information about a fragment.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Span<T> {
    pub fragment: T,
    pub line: usize,
    pub offset: usize,
}


/**
 * Implements Span function.
 */
impl<T: Clone> Span<T> {
    /**
     * Creates a new Span.
     */
    pub fn new(fragment: T, line: usize, offset: usize) -> Span<T> {
        Span{
            fragment: fragment,
            line: line,
            offset: offset,
        }
    }


    /**
     * Returns fragment.
     */
    pub fn get_fragment(&self) -> T {
        return self.fragment.clone();
    }


    /**
     * Returns the line number.
     */
    pub fn get_line(&self) -> usize {
        return self.line;
    }


    /**
     * Returns the offset number.
     */
    pub fn get_offset(&self) -> usize {
        return self.offset;
    }
}

