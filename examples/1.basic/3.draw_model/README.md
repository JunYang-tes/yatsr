## 运行
```
cargo run --example basic_draw_model
```
该命令会在当前工作目录下生成basic_draw_model.ppm图片。结果如下图，因为没有考虑三角形的深度（z坐标），所有不同深度的三角形的绘制结果相互干扰，结果看起来会很不对。

![result](./basic_draw_model.png)

另一个模型：

![result](./spot.png)