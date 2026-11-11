use std::str::FromStr;

use mongodb::bson::{Document, doc};

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortField {
    #[default]
    Name,
    Gpa,
    Class,
    EnrolledAt,
}

impl SortField {
    pub fn field(self) -> &'static str {
        match self {
            SortField::Name => "name",
            SortField::Gpa => "gpa",
            SortField::Class => "class",
            SortField::EnrolledAt => "is_enrolled",
        }
    }
}

impl FromStr for SortField {
    type Err = AppError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.to_lowercase().as_str() {
            "name" => Ok(SortField::Name),
            "gpa" => Ok(SortField::Gpa),
            "class" => Ok(SortField::Class),
            "enrolled_at" | "enrolled" => Ok(SortField::EnrolledAt),
            other => Err(AppError::Validation(format!("unknown sort field: {other}"))),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn direction(self) -> i32 {
        match self {
            SortOrder::Ascending => 1,
            SortOrder::Descending => -1,
        }
    }
}

impl FromStr for SortOrder {
    type Err = AppError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.to_lowercase().as_str() {
            "asc" | "ascending" => Ok(SortOrder::Ascending),
            "desc" | "descending" => Ok(SortOrder::Descending),
            other => Err(AppError::Validation(format!("unknown sort order: {other}"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paging {
    page: u64,
    per_page: u32,
}

impl Paging {
    pub const DEFAULT_PER_PAGE: u32 = 10;
    pub const MAX_PER_PAGE: u32 = 100;

    pub fn new(page: u64, per_page: u32) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, Self::MAX_PER_PAGE),
        }
    }

    pub fn page(&self) -> u64 {
        self.page
    }

    pub fn per_page(&self) -> u32 {
        self.per_page
    }

    pub fn skip(&self) -> u64 {
        (self.page - 1).saturating_mul(u64::from(self.per_page))
    }

    pub fn limit(&self) -> i64 {
        i64::from(self.per_page)
    }
}

impl Default for Paging {
    fn default() -> Self {
        Self::new(1, Self::DEFAULT_PER_PAGE)
    }
}

#[expect(dead_code, reason = "TODO")]
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub page: u64,
    pub per_page: u32,
    pub total: u64,
}

impl<T> Page<T> {
    #[expect(dead_code, reason = "TODO")]
    pub fn total_pages(&self) -> u64 {
        self.total.div_ceil(u64::from(self.per_page))
    }
}

#[derive(Debug, Clone, Default)]
pub struct StudentFilter {
    pub class: Option<String>,
    pub is_active: Option<bool>,
    pub city: Option<String>,
    pub name_contains: Option<String>,
    pub min_gpa: Option<f64>,
    pub sort_by: SortField,
    pub order: SortOrder,
    pub paging: Paging,
}

impl StudentFilter {
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    #[expect(dead_code, reason = "TODO")]
    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    #[expect(dead_code, reason = "TODO")]
    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    #[expect(dead_code, reason = "TODO")]
    pub fn name_contains(mut self, text: impl Into<String>) -> Self {
        self.name_contains = Some(text.into());
        self
    }

    #[expect(dead_code, reason = "TODO")]
    pub fn min_gpa(mut self, gpa: f64) -> Self {
        self.min_gpa = Some(gpa);
        self
    }

    pub fn sort(mut self, field: SortField, order: SortOrder) -> Self {
        self.sort_by = field;
        self.order = order;
        self
    }

    pub fn page(mut self, page: u64) -> Self {
        self.paging = Paging::new(page, self.paging.per_page());
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.paging = Paging::new(self.paging.page(), per_page);
        self
    }
    pub fn to_query(&self) -> Document {
        let mut query = Document::new();

        if let Some(class) = &self.class {
            query.insert("class", class.as_str());
        }
        if let Some(is_active) = self.is_active {
            query.insert("is_active", is_active);
        }
        if let Some(city) = &self.city {
            query.insert("address.city", city.as_str());
        }
        if let Some(text) = &self.name_contains {
            let pattern = escape_regex(text);
            query.insert("name", doc! { "$regex": pattern, "$options": "i" });
        }
        if let Some(min_gpa) = self.min_gpa {
            query.insert("gpa", doc! { "$gte": min_gpa });
        }
        query
    }

    pub fn to_sort(&self) -> Document {
        let mut sort = Document::new();
        sort.insert(self.sort_by.field(), self.order.direction());
        sort.insert("_id", 1);
        sort
    }
}
    fn escape_regex(text: &str) -> String {
        const SPECIAL: &str = r"\^$.|?*+()[]{}";

        let mut escaped = String::with_capacity(text.len());
        for character in text.chars() {
            if SPECIAL.contains(character) {
                escaped.push('\\');
            }
            escaped.push(character);
        }
        escaped
    }

