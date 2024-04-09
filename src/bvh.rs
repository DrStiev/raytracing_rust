use crate::{util::*, HitRecord, Hittable, HittableList, Interval, Ray, AABB};
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(list: HittableList) -> Self {
        let mut objects = list.objects.clone();
        let axis: i32 = random_double_in_range(0.0, 2.0) as i32;

        objects.sort_by(|a, b| Self::box_compare(a, b, axis as usize));

        let object_span: usize = objects.len();
        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = if object_span == 1 {
            (Arc::clone(&objects[0]), Arc::clone(&objects[0]))
        } else if object_span == 2 {
            (Arc::clone(&objects[0]), Arc::clone(&objects[1]))
        } else {
            let mid = object_span / 2;
            let mut h1 = HittableList::new();
            for obj in objects[0..mid].iter() {
                h1.add(*obj);
            }

            let mut h2 = HittableList::new();
            for obj in objects[mid..].iter() {
                h2.add(*obj);
            }
            (Arc::new(BVHNode::new(h1)), Arc::new(BVHNode::new(h2)))
        };

        let bbox = AABB::new_with_aabb(left.bounding_box(), right.bounding_box());
        Self { left, right, bbox }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        a.bounding_box()
            .axis(axis_index)
            .min
            .partial_cmp(&b.bounding_box().axis(axis_index).min)
            .unwrap_or(Ordering::Equal)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if self.bbox.hit(&r, ray_t) {
            return false;
        }
        let hit_left: bool = self.left.hit(r, ray_t, rec);
        let hit_right: bool = self.right.hit(r, ray_t, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
