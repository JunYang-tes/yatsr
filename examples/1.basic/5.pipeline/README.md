## 渲染管线

下图来自OpenGL
![render pipeline](./OpenGL-Rendering-Pipeline.png)

在这里，我们将Vertex Processor 和Fragment Processor 抽象成Shader Trait 里面的两个函数。有render 函数完成整个流程，render函数使用不同的shader渲染不的效果:

![result](./flat_lambert.png)![result](./lambert.png)
![result](./flat_ball.png)![result](./ball.png)
