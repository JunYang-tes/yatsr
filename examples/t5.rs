use yatsr::{geometry::Vec3, mat::Mat4};

fn main() {
    let near = -2.;
    let far = -4.;

    let m = Mat4([
      near, 0.,  0.,  0.,
      0.,near ,  0.,  0.,
      0.,0.,  near+far ,-far*near,
      0.,0.,  1.,  0.
    ]);
    println!("{:?}",&m * &Vec3::new(0.,2.,-2.));
    println!("{:?}",&m * &Vec3::new(0.,2.,-4.));
    println!("{:?}",&m * &Vec3::new(0.,2.,-1.));
}
