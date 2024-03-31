## 运行
```
cargo run --example basic_draw_model
```
该命令会在当前工作目录下生成basic_draw_model.ppm图片。结果如下图，因为没有考虑三角形的深度（z坐标），所有不同深度的三角形的绘制结果相互干扰，结果只能大概看出一个“人”的形状

![result](./basic_draw_model.png)