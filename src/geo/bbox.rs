//! This module contains an implementation of a [Minimum bounding
//! box](https://en.wikipedia.org/wiki/Minimum_bounding_box) or AABB.

extern crate num;

use std::iter::FromIterator;

use geo::Point;

/// Simple axis aligned bounding box implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox<T> {
    min: Point<T>,
    max: Point<T>,
}

impl<T> BoundingBox<T>
where
    T: num::Num + num::Bounded + From<u8> + Copy + PartialOrd,
{
    /// Create a new empty BoundingBox.
    pub fn new() -> Self {
        Self {
            min: Point::new(T::max_value(), T::max_value()),
            max: Point::new(T::min_value(), T::min_value()),
        }
    }

    /// Create a new BoundingBox of the given width and height starting from the
    /// origin.
    pub fn from_dimensions(width: T, height: T) -> Self {
        Self::from_dimensions_and_origin(&Point::new(T::from(0), T::from(0)), width, height)
    }

    /// Create a new BoundingBox of the given width and height starting from the
    /// origin.
    pub fn from_dimensions_and_origin(origin: &Point<T>, width: T, height: T) -> Self {
        let mut bbox = Self::new();

        bbox.expand_by_point(origin);
        bbox.expand_by_point(&Point::new(origin.x + width, origin.y + height));

        bbox
    }

    /// Return the point with the lowest coordinates.
    pub fn min(&self) -> &Point<T> {
        &self.min
    }

    /// Return the point with the highest coordinates.
    pub fn max(&self) -> &Point<T> {
        &self.max
    }

    /// Expand this bounding box by the given point.
    pub fn expand_by_point(&mut self, pt: &Point<T>) {
        self.min = self.min.lowest(pt);
        self.max = self.max.highest(pt);
    }

    /// Check if a point lies inside this bounding box.
    pub fn contains(&self, pt: &Point<T>) -> bool {
        self.min.x <= pt.x && self.max.x >= pt.x && self.min.y <= pt.y && self.max.y >= pt.y
    }

    /// Return the points of this rectangle in clockwise order.
    pub fn points(&self) -> [Point<T>; 4] {
        [
            self.min,
            Point::new(self.max.x, self.min.y),
            self.max,
            Point::new(self.min.x, self.max.y),
        ]
    }

    /// Return the center of this rectangle.
    pub fn center(&self) -> Point<T> {
        Point::new(
            (self.min.x + self.max.x) / T::from(2),
            (self.min.y + self.max.y) / T::from(2),
        )
    }
}

impl<T> Default for BoundingBox<T>
where
    T: num::Num + num::Bounded + From<u8> + Copy + PartialOrd,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<Point<T>> for BoundingBox<T>
where
    T: num::Num + num::Bounded + From<u8> + Copy + PartialOrd,
{
    fn from_iter<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point<T>>,
    {
        let mut bbox = BoundingBox::new();

        for point in points {
            bbox.expand_by_point(&point);
        }

        bbox
    }
}

#[cfg(test)]
mod test {
    use super::BoundingBox;

    use geo::PointU32;

    #[test]
    fn test_contains() {
        let rec = BoundingBox::from_dimensions_and_origin(&PointU32::new(3, 5), 7, 5);

        assert_eq!(rec.contains(&PointU32::new(0, 0)), false);
        assert_eq!(rec.contains(&PointU32::new(4, 0)), false);
        assert_eq!(rec.contains(&PointU32::new(0, 8)), false);
        assert_eq!(rec.contains(&PointU32::new(40, 40)), false);

        assert_eq!(rec.contains(&PointU32::new(3, 5)), true);
        assert_eq!(rec.contains(&PointU32::new(5, 7)), true);
        assert_eq!(rec.contains(&PointU32::new(10, 10)), true);
    }

    #[test]
    fn test_points() {
        let rec = BoundingBox::from_dimensions_and_origin(&PointU32::new(3, 5), 7, 5);

        assert_eq!(
            rec.points(),
            [
                PointU32::new(3, 5),
                PointU32::new(10, 5),
                PointU32::new(10, 10),
                PointU32::new(3, 10),
            ]
        )
    }

    #[test]
    fn test_center() {
        let rec = BoundingBox::from_dimensions_and_origin(&PointU32::new(2, 4), 8, 6);

        assert_eq!(rec.center(), PointU32::new(6, 7));
    }
}