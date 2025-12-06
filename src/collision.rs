use macroquad::prelude::*;

/// Represents the shape of a collidable entity
#[derive(Debug, Clone, Copy)]
pub enum Collider {
    Circle {
        radius: f32,
    },
    #[allow(dead_code)]
    Rect {
        width: f32,
        height: f32,
    }, // AABB (Axis-Aligned Bounding Box), centered
}

/// Trait for entities that can participate in collision detection
pub trait Collidable {
    fn collider(&self) -> Collider;
    fn position(&self) -> Vec2;
}

/// Contains detailed information about a collision
#[derive(Debug, Clone, Copy)]
pub struct CollisionData {
    pub collided: bool,
    #[allow(dead_code)]
    pub penetration_depth: f32,
    pub normal: Vec2, // Points from object 2 to object 1
}

impl CollisionData {
    pub fn none() -> Self {
        Self {
            collided: false,
            penetration_depth: 0.0,
            normal: Vec2::ZERO,
        }
    }

    pub fn new(penetration_depth: f32, normal: Vec2) -> Self {
        Self {
            collided: true,
            penetration_depth,
            normal,
        }
    }
}

/// Check collision between two collidable entities
pub fn check_collision(
    collider1: &Collider,
    pos1: Vec2,
    collider2: &Collider,
    pos2: Vec2,
) -> CollisionData {
    match (collider1, collider2) {
        (Collider::Circle { radius: r1 }, Collider::Circle { radius: r2 }) => {
            circle_circle(pos1, *r1, pos2, *r2)
        }
        (Collider::Circle { radius }, Collider::Rect { width, height }) => {
            circle_rect(pos1, *radius, pos2, *width, *height)
        }
        (Collider::Rect { width, height }, Collider::Circle { radius }) => {
            // Reverse collision and flip normal
            let mut result = circle_rect(pos2, *radius, pos1, *width, *height);
            result.normal = -result.normal;
            result
        }
        (
            Collider::Rect {
                width: w1,
                height: h1,
            },
            Collider::Rect {
                width: w2,
                height: h2,
            },
        ) => rect_rect(pos1, *w1, *h1, pos2, *w2, *h2),
    }
}

/// Check collision between two circles
fn circle_circle(pos1: Vec2, r1: f32, pos2: Vec2, r2: f32) -> CollisionData {
    let delta = pos1 - pos2;
    let distance_sq = delta.length_squared();
    let radii_sum = r1 + r2;
    let radii_sum_sq = radii_sum * radii_sum;

    if distance_sq < radii_sum_sq {
        let distance = distance_sq.sqrt();
        let penetration = radii_sum - distance;

        // Normal points from pos2 to pos1
        let normal = if distance > 0.0001 {
            delta / distance
        } else {
            // Circles are at same position, use arbitrary normal
            Vec2::new(1.0, 0.0)
        };

        CollisionData::new(penetration, normal)
    } else {
        CollisionData::none()
    }
}

/// Check collision between a circle and an axis-aligned rectangle (centered)
fn circle_rect(
    circle_pos: Vec2,
    radius: f32,
    rect_pos: Vec2,
    width: f32,
    height: f32,
) -> CollisionData {
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    // Find the closest point on the rectangle to the circle's center
    let closest_x = circle_pos
        .x
        .clamp(rect_pos.x - half_width, rect_pos.x + half_width);
    let closest_y = circle_pos
        .y
        .clamp(rect_pos.y - half_height, rect_pos.y + half_height);
    let closest_point = Vec2::new(closest_x, closest_y);

    let delta = circle_pos - closest_point;
    let distance_sq = delta.length_squared();

    if distance_sq < radius * radius {
        let distance = distance_sq.sqrt();
        let penetration = radius - distance;

        // Normal points from rect to circle
        let normal = if distance > 0.0001 {
            delta / distance
        } else {
            // Circle center is inside rect, find normal based on closest edge
            let dx_left = (circle_pos.x - (rect_pos.x - half_width)).abs();
            let dx_right = ((rect_pos.x + half_width) - circle_pos.x).abs();
            let dy_top = (circle_pos.y - (rect_pos.y - half_height)).abs();
            let dy_bottom = ((rect_pos.y + half_height) - circle_pos.y).abs();

            let min_dist = dx_left.min(dx_right).min(dy_top).min(dy_bottom);

            if min_dist == dx_left {
                Vec2::new(-1.0, 0.0)
            } else if min_dist == dx_right {
                Vec2::new(1.0, 0.0)
            } else if min_dist == dy_top {
                Vec2::new(0.0, -1.0)
            } else {
                Vec2::new(0.0, 1.0)
            }
        };

        CollisionData::new(penetration, normal)
    } else {
        CollisionData::none()
    }
}

/// Check collision between two axis-aligned rectangles (centered)
fn rect_rect(pos1: Vec2, w1: f32, h1: f32, pos2: Vec2, w2: f32, h2: f32) -> CollisionData {
    let half_w1 = w1 / 2.0;
    let half_h1 = h1 / 2.0;
    let half_w2 = w2 / 2.0;
    let half_h2 = h2 / 2.0;

    let delta = pos1 - pos2;
    let overlap_x = (half_w1 + half_w2) - delta.x.abs();
    let overlap_y = (half_h1 + half_h2) - delta.y.abs();

    if overlap_x > 0.0 && overlap_y > 0.0 {
        // Collision detected, find minimum penetration axis
        let (penetration, normal) = if overlap_x < overlap_y {
            // Separate along X axis
            let normal_x = if delta.x > 0.0 { 1.0 } else { -1.0 };
            (overlap_x, Vec2::new(normal_x, 0.0))
        } else {
            // Separate along Y axis
            let normal_y = if delta.y > 0.0 { 1.0 } else { -1.0 };
            (overlap_y, Vec2::new(0.0, normal_y))
        };

        CollisionData::new(penetration, normal)
    } else {
        CollisionData::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_circle_collision() {
        let pos1 = Vec2::new(0.0, 0.0);
        let pos2 = Vec2::new(5.0, 0.0);
        let result = circle_circle(pos1, 3.0, pos2, 3.0);
        assert!(result.collided);
        assert_eq!(result.penetration_depth, 1.0);
    }

    #[test]
    fn test_circle_circle_no_collision() {
        let pos1 = Vec2::new(0.0, 0.0);
        let pos2 = Vec2::new(10.0, 0.0);
        let result = circle_circle(pos1, 3.0, pos2, 3.0);
        assert!(!result.collided);
    }

    #[test]
    fn test_rect_rect_collision() {
        let pos1 = Vec2::new(0.0, 0.0);
        let pos2 = Vec2::new(5.0, 0.0);
        let result = rect_rect(pos1, 6.0, 6.0, pos2, 6.0, 6.0);
        assert!(result.collided);
        assert_eq!(result.penetration_depth, 1.0);
    }

    #[test]
    fn test_circle_rect_collision() {
        let circle_pos = Vec2::new(0.0, 0.0);
        let rect_pos = Vec2::new(3.0, 0.0);
        let result = circle_rect(circle_pos, 3.0, rect_pos, 4.0, 4.0);
        assert!(result.collided);
    }
}
