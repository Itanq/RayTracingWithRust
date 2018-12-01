use std::ops::{ Add };

#[derive(Debug)]
pub struct Ray {
    pub original: Vec3,
    pub direction: Vec3,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    pub point: (f32, f32, f32),
}

impl Vec3 {

    pub fn add(&self, right: &Self) -> Self {
        Vec3 {
            point: ( self.point.0 + right.point.0,
            self.point.1 + right.point.1,
            self.point.2 + right.point.2 )
        }
    }

    pub fn mul(&self, scala: f32) -> Self {
        Vec3 {
            point: (self.point.0 * scala, self.point.1 * scala, self.point.2 * scala)
        }
    }

    pub fn div(&self, scala: f32) -> Self {
        Vec3 {
            point: (self.point.0 / scala, self.point.1 / scala, self.point.2 / scala)
        }
    }

    pub fn unit(&self) -> Self {
        self.div(3.0f32)
    }
}



impl Ray {

    pub fn get_point(self, t: f32) -> Vec3 {
        let res = self.original.add(&self.direction.mul(t));
        res
    }

    pub fn color(&self) -> Vec3 {
        let unit_direction = self.direction.unit();
        let t = 0.5f32 * (unit_direction.point.1 + 1.0f32);
        let a = Vec3{ point:(1.0, 1.0, 1.0) };
        let b = Vec3{ point:(0.5, 0.7, 1.0) };
        a.mul(1.0 - t).add(&b.mul(t))
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
