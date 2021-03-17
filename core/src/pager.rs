/**
 * This `struct` is created by the [`Connection::paginate_find_where`] method.
 *
 * [`Connection::paginate_find_where`]: crate::Connection::paginate_find_where
 */
#[derive(Debug)]
pub struct Pager<E: crate::Entity> {
    rows: crate::Rows<E>,
    count: usize,
    page: usize,
    max_per_page: usize,
}

impl<E: crate::Entity> Pager<E> {
    /**
     * Creates a new pager.
     *
     * `page` starts at 1.
     */
    pub fn new(
        rows: crate::Rows<E>,
        count: usize,
        page: usize,
        max_per_page: usize,
    ) -> Self {
        Self {
            rows,
            count,
            page,
            max_per_page,
        }
    }

    /**
     * Get the number of results in this page.
     */
    pub fn result_count(&self) -> usize {
        self.rows.len()
    }

    /**
     * Get the index of the first element of this page.
     */
    pub fn result_min(&self) -> usize {
        usize::min(1 + self.max_per_page * (self.page - 1), self.count)
    }

    /**
     * Get the index of the last element of this page.
     */
    pub fn result_max(&self) -> usize {
        (self.page - 1) * self.max_per_page + self.result_count()
    }

    /**
     * Get the last page index.
     */
    pub fn last_page(&self) -> usize {
        if self.count == 0 {
            1
        }
        else {
            (self.count as f32 / self.max_per_page as f32).ceil() as usize
        }
    }

    /**
     * Get the current page index.
     */
    pub fn page(&self) -> usize {
        self.page
    }

    /**
     * True if a next page exists.
     */
    pub fn has_next_page(&self) -> bool {
        self.page < self.last_page()
    }

    /**
     * True if a previous page exists.
     */
    pub fn has_previous_page(&self) -> bool {
        self.page > 1
    }

    /**
     * Get the total number of results in all pages.
     */
    pub fn count(&self) -> usize {
        self.count
    }

    /**
     * Get maximum result per page.
     */
    pub fn max_per_page(&self) -> usize {
        self.max_per_page
    }

    /**
     * Get results rows.
     */
    pub fn rows(&self) -> &crate::Rows<E> {
        &self.rows
    }
}

impl<E: crate::Entity> std::iter::IntoIterator for Pager<E> {
    type IntoIter = crate::Rows<Self::Item>;
    type Item = E;

    fn into_iter(self) -> Self::IntoIter {
        self.rows
    }
}

#[cfg(feature = "serde")]
impl<E: crate::Entity + serde::Serialize> serde::Serialize for Pager<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Pager", 3)?;

        state.serialize_field("result_count", &self.result_count())?;
        state.serialize_field("result_min", &self.result_min())?;
        state.serialize_field("result_max", &self.result_max())?;
        state.serialize_field("last_page", &self.last_page())?;
        state.serialize_field("page", &self.page())?;
        state.serialize_field("has_next_page", &self.has_next_page())?;
        state
            .serialize_field("has_previous_page", &self.has_previous_page())?;
        state.serialize_field("count", &self.count())?;
        state.serialize_field("max_per_page", &self.max_per_page())?;
        state.serialize_field("iterator", &self.rows)?;

        state.end()
    }
}
