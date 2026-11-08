//! Foundational geometric primitives and domain types for OMNISCAN.

use serde::{Deserialize, Serialize};

/// A point in 2D space using normalized coordinates [0.0, 1.0].
/// Origin (0.0, 0.0) is at the top-left corner of the page.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Checks if point coordinates are within normalized [0.0, 1.0] range.
    pub fn is_normalized(&self) -> bool {
        (0.0..=1.0).contains(&self.x) && (0.0..=1.0).contains(&self.y)
    }

    /// Computes Euclidean distance to another point.
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A 2D dimension specification with normalized width and height [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size2D {
    pub width: f64,
    pub height: f64,
}

impl Size2D {
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn is_positive(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }

    pub fn is_normalized(&self) -> bool {
        self.width > 0.0 && self.width <= 1.0 && self.height > 0.0 && self.height <= 1.0
    }
}

/// A rectangular region in normalized coordinate space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NormalizedRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl NormalizedRect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Validates that dimensions are strictly positive and rectangle lies fully within [0.0, 1.0].
    pub fn is_within_bounds(&self) -> bool {
        self.width > 0.0
            && self.height > 0.0
            && self.x >= 0.0
            && self.y >= 0.0
            && (self.x + self.width) <= 1.0 + 1e-9
            && (self.y + self.height) <= 1.0 + 1e-9
    }

    /// Checks if a point lies inside this rectangle.
    pub fn contains_point(&self, p: &Point2D) -> bool {
        p.x >= self.x && p.x <= self.x + self.width && p.y >= self.y && p.y <= self.y + self.height
    }
}

/// A circular bubble or marker in normalized coordinate space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub center: Point2D,
    pub radius: f64,
}

impl Circle {
    pub const fn new(center: Point2D, radius: f64) -> Self {
        Self { center, radius }
    }

    /// Validates that radius is positive and the circle lies completely within [0.0, 1.0].
    pub fn is_within_bounds(&self) -> bool {
        self.radius > 0.0
            && (self.center.x - self.radius) >= -1e-9
            && (self.center.x + self.radius) <= 1.0 + 1e-9
            && (self.center.y - self.radius) >= -1e-9
            && (self.center.y + self.radius) <= 1.0 + 1e-9
    }

    /// Checks if this circle overlaps with another circle, using the sum of their radii.
    pub fn overlaps_with(&self, other: &Circle) -> bool {
        let dist = self.center.distance_to(&other.center);
        let min_distance = self.radius + other.radius;
        // Strict overlap: distance is less than the sum of radii (allowing small epsilon for contact)
        dist < (min_distance - 1e-7)
    }
}

/// Page orientation definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_normalized() {
        let p1 = Point2D::new(0.5, 0.5);
        assert!(p1.is_normalized());

        let p2 = Point2D::new(1.1, 0.5);
        assert!(!p2.is_normalized());

        let p3 = Point2D::new(0.0, 0.0);
        let p4 = Point2D::new(0.03, 0.04);
        assert!((p3.distance_to(&p4) - 0.05).abs() < 1e-9);
    }

    #[test]
    fn test_rect_bounds() {
        let r1 = NormalizedRect::new(0.1, 0.1, 0.5, 0.5);
        assert!(r1.is_within_bounds());

        let r2 = NormalizedRect::new(0.8, 0.8, 0.3, 0.1);
        assert!(!r2.is_within_bounds());

        let r3 = NormalizedRect::new(-0.01, 0.1, 0.2, 0.2);
        assert!(!r3.is_within_bounds());
    }

    #[test]
    fn test_circle_bounds() {
        let c1 = Circle::new(Point2D::new(0.5, 0.5), 0.1);
        assert!(c1.is_within_bounds());

        let c2 = Circle::new(Point2D::new(0.05, 0.5), 0.06);
        assert!(!c2.is_within_bounds());

        let c3 = Circle::new(Point2D::new(0.5, 0.5), -0.01);
        assert!(!c3.is_within_bounds());
    }

    #[test]
    fn test_circle_overlap_with_different_radii() {
        // c1: radius 0.05 at (0.2, 0.2)
        // c2: radius 0.03 at (0.26, 0.2) -> distance is 0.06, sum of radii is 0.08 -> overlap!
        let c1 = Circle::new(Point2D::new(0.2, 0.2), 0.05);
        let c2 = Circle::new(Point2D::new(0.26, 0.2), 0.03);
        assert!(c1.overlaps_with(&c2));

        // c3: radius 0.03 at (0.30, 0.2) -> distance is 0.10, sum of radii is 0.08 -> no overlap
        let c3 = Circle::new(Point2D::new(0.30, 0.2), 0.03);
        assert!(!c1.overlaps_with(&c3));
    }
}
