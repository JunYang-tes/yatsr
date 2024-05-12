use std::convert::Into;
use std::ops::*;
macro_rules! Vecs {
  ($name:ident,$N:literal,$($components:ident,)*) => {
    #[derive(Debug,Clone,Copy)]
    pub struct $name<T: Copy>{
        $(pub $components:T,)*
    }
    impl<T: Copy> $name<T> {
        pub fn new($($components:T,)*)->$name<T> {
            $name {
                $($components,)*
            }
        }
        $(pub fn $components (&self)->T{
            self.$components
        })*
    }
    impl<T> $name<T> where
        T: Mul + Copy,
        <T as Mul>::Output: Copy
    {
        pub fn components_mul(self,rhs:$name<T>)->$name<<T as Mul>::Output> {
            $name::new(
                $(self.$components*rhs.$components,)*
            )
        }
    }
    impl<T:Copy+ Default> Default for $name<T> {
        fn default()->$name<T> {
            $name {
                $($components: T::default(),)*
            }
        }

    }
    impl Into<$name<f32>> for &$name<i32> {
        fn into(self)-> $name<f32> {
            $name::new($(self.$components as f32,)*)
        }
    }
    impl Into<$name<f32>> for &$name<u32> {
        fn into(self)-> $name<f32> {
            $name::new($(self.$components as f32,)*)
        }
    }
    impl Into<$name<f32>> for &$name<u8> {
        fn into(self)-> $name<f32> {
            $name::new($(self.$components as f32,)*)
        }
    }
    impl Into<$name<f32>> for $name<u8> {
        fn into(self)-> $name<f32> {
            $name::new($(self.$components as f32,)*)
        }
    }
    impl Into<$name<f32>> for $name<u32> {
        fn into(self)-> $name<f32> {
            $name::new($(self.$components as f32,)*)
        }
    }
    impl Into<$name<u32>> for $name<f32> {
        fn into(self)-> $name<u32> {
            $name::new($(self.$components as u32,)*)
        }
    }


    impl<T> Add for $name<T>
    where
     T: Add + Copy,
     <T as Add>::Output:Copy,
     {
         type Output=$name<T::Output>;
         fn add(self,rhs:Self)-> Self::Output {
             $name::new($(self.$components+rhs.$components,)*)
         }
    }
    impl<T> Add for &$name<T>
    where
     T: Add + Copy,
     <T as Add>::Output:Copy,
     {
         type Output=$name<T::Output>;
         fn add(self,rhs:Self)-> Self::Output {
             $name::new($(self.$components+rhs.$components,)*)
         }
    }

    impl<T> Sub for $name<T>
    where
     T: Sub + Copy,
     <T as Sub>::Output:Copy,
     {
         type Output=$name<T::Output>;
         fn sub(self,rhs:Self)-> Self::Output {
             $name::new($(self.$components-rhs.$components,)*)
         }
    }
    impl<T> Sub for &$name<T>
    where
     T: Sub + Copy,
     <T as Sub>::Output:Copy,
     {
         type Output=$name<T::Output>;
         fn sub(self,rhs:Self)-> Self::Output {
             $name::new($(self.$components-rhs.$components,)*)
         }
    }
    // vec * number
    impl<T> Mul<T> for $name<T>
    where
      T:Mul + Copy,
      <T as Mul>::Output:Copy
    {
          type Output=$name<<T as Mul>::Output>;
          fn mul(self,rhs:T)->Self::Output {
            $name::new($(self.$components*rhs,)*)
          }
    }
    // vec * number
    impl<T> Mul<T> for &$name<T>
    where
      T:Mul + Copy,
      <T as Mul>::Output:Copy
    {
          type Output=$name<<T as Mul>::Output>;
          fn mul(self,rhs:T)->Self::Output {
            $name::new($(self.$components*rhs,)*)
          }
    }
    // vec * vec
    impl<T> Mul for $name<T>
    where
      T:Add<Output=T>+Mul<Output=T>+Copy,
    {
        type Output=T;
        fn mul(self,rhs:Self) -> Self::Output {
            let mut items = Vec::<<T as Mul>::Output>::new();
            $(
                items.push(self.$components*rhs.$components);
             )*
            let mut r = items[0]+items[1];
            for i in 2..items.len() {
                r=r+items[i];
            }
            r
        }
    }
    // vec * vec
    impl<T> Mul for &$name<T>
    where
      T:Add<Output=T>+Mul<Output=T>+Copy,
    {
        type Output=T;
        fn mul(self,rhs:Self) -> Self::Output {
            let mut items = Vec::<<T as Mul>::Output>::new();
            $(
                items.push(self.$components*rhs.$components);
             )*
            let mut r = items[0]+items[1];
            for i in 2..items.len() {
                r=r+items[i];
            }
            r
        }
    }


  };
}
Vecs!(Vec2, 2, x, y,);
Vecs!(Vec3, 3, x, y, z,);

impl<T: Copy> Vec2<T>
where
  T: Mul<Output = T> + Sub<Output = T> + Into<f32>,
{
  pub fn norm(self) -> f32 {
    let x: f32 = self.x.into();
    let y: f32 = self.y.into();
    (x * x + y * y).sqrt().into()
  }
  pub fn normalize(self) -> Vec2<f32> {
    let x: f32 = self.x.into();
    let y: f32 = self.y.into();
    let l = self.norm();
    let n = Vec2::new(x, y);
    n * (1. / l)
  }
}

impl<T: Copy> Vec3<T>
where
  T: Mul<Output = T> + Sub<Output = T> + Into<f32>,
{
  pub fn cross_product(self, rhs: Self) -> Vec3<T> {
    // y*v.z-z*v.y, z*v.x-x*v.z, x*v.y-y*v.x
    let x = self.y * rhs.z - self.z * rhs.y;
    let y = self.z * rhs.x - self.x * rhs.z;
    let z = self.x * rhs.y - self.y * rhs.x;
    Vec3::new(x, y, z)
  }
  pub fn product(self, rhs: Self) -> Vec3<T> {
    Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * self.z)
  }
  pub fn norm(self) -> f32 {
    let x: f32 = self.x.into();
    let y: f32 = self.y.into();
    let z: f32 = self.z.into();
    (x * x + y * y + z * z).sqrt().into()
  }
  pub fn normalize(self) -> Vec3<f32> {
    let x: f32 = self.x.into();
    let y: f32 = self.y.into();
    let z: f32 = self.z.into();
    let l = self.norm();
    let n = Vec3::new(x, y, z);
    n * (1. / l)
  }
}
impl<T: Copy> Index<u8> for Vec3<T> {
  type Output = T;
  fn index(&self, index: u8) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      _ => &self.z,
    }
  }
}
impl<T: Copy> IndexMut<u8> for Vec3<T> {
  fn index_mut(&mut self, index: u8) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      _ => &mut self.z,
    }
  }
}
Vecs!(Vec4, 4, x, y, z, w,);
impl Vec4<f32> {
  pub fn from_point(p: &Vec3<f32>) -> Vec4<f32> {
    Vec4::new(p.x, p.y, p.z, 1.)
  }
  pub fn from_vector(v: &Vec3<f32>) -> Vec4<f32> {
    Vec4::new(v.x, v.y, v.z, 0.)
  }
  pub fn to_3d_point(&self) -> Vec3<f32> {
    Vec3::new(self.x / self.w, self.y / self.w, self.z / self.w)
  }
  pub fn to_3d_vector(&self) -> Vec3<f32> {
    Vec3::new(self.x, self.y, self.z)
  }
  pub fn set(&mut self, i: u8, v: f32) {
    match i {
      0 => self.x = v,
      1 => self.y = v,
      2 => self.z = v,
      3 => self.w = v,
      _ => {
        panic!("Index out of range")
      }
    }
  }
}
