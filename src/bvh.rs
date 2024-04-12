use crate::aabb;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BVHNode {
    // use dyn to make explicit the dynamic dispatch of an object
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hittable: &mut [Rc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        // FnMut trait is used for types that can be called as they were functions
        // and are mutable (in this case)
        fn box_compare(
            time0: f64,
            time1: f64,
            axis: usize,
        ) -> impl FnMut(&Rc<dyn Hittable>, &Rc<dyn Hittable>) -> Ordering {
            // move converts any variables captured by reference or mutable reference to variables captured by value.
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if a_bbox.is_none() || b_bbox.is_none() {
                    panic!["No bounding box in BVHNode"]
                }
                if a_bbox.unwrap().min[axis] - b_bbox.unwrap().min[axis] < 0.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }

        let axis = rand::thread_rng().gen_range(0..3) as usize;

        hittable.sort_unstable_by(box_compare(time0, time1, axis));
        let len = hittable.len();
        let (left, right) = if len == 1 {
            (hittable[0].clone(), hittable[0].clone())
        } else if len == 2 {
            (hittable[0].clone(), hittable[1].clone())
        } else {
            (
                Rc::new(BVHNode::new(&mut hittable[0..len / 2], time0, time1)) as Rc<dyn Hittable>,
                Rc::new(BVHNode::new(&mut hittable[len / 2..len], time0, time1))
                    as Rc<dyn Hittable>,
            )
        };
        let left_bbox = left.bounding_box(time0, time1);
        let right_box = right.bounding_box(time0, time1);
        if left_bbox.is_none() || right_box.is_none() {
            panic!["No bounding box in BVHNode"]
        }

        BVHNode {
            left,
            right,
            bbox: aabb::surrounding_box(&left_bbox.unwrap(), &right_box.unwrap()),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(&ray, t_min, t_max) {
            let left = self.left.hit(&ray, t_min, t_max);
            let right = self.right.hit(&ray, t_min, t_max);
            match (left, right) {
                (Some(l), Some(r)) => {
                    if l.t < r.t {
                        Some(l)
                    } else {
                        Some(r)
                    }
                }
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
