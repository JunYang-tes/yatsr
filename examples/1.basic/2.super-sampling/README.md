通过将一个像素划分为四个子像素（超采样，super sampling)来反锯齿。
```shell
cargo run --example=basic_super_sampling
```

对比：

![aliasing](../1.triangle/colorfull_triangle.png)![anti-aliasing](./super-sample.png)

局部：

![aliasing](./aliasing.png)![anti-aliasing](./anti-aliasing-local.png)
