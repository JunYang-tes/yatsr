

## 三维空间的基本旋转

![rotation](./rotation.svg)

三维空间有三种基本的旋转，分别为绕三个轴的旋转。上图展示了绕三个轴旋转的正方向。z轴垂直于屏幕（或渲染图片）向外。

![](./output.png)
> 绕三个轴旋转45度效果

### 绕z轴旋转

绕z轴旋转时，z坐标保持不变。x,y坐标的旋转方式和2D空间相同。

![](./rotate_z_1.svg)
<!--
{ (x'=cos(alpha)x-sin(alpha)y),(y'=sin(alpha)x+cos(alpha)y):}
-->

写成矩阵形式：

![rotate z](./rotate-z.svg)

<!--
[[cos(alpha),-sin(alpha),0,0],
 [sin(alpha),cos(alpha),0,0],
 [0         ,0         ,1,0],
 [0         ,0         ,0,1]]
((x),(y),(z),(1)) = 
((x'),(y'),(z'),(1))
-->


### 绕y轴旋转

绕y轴旋转的时候，y坐标保持不变。xoz 平面上旋转的正方向于xoy平面的正方向正好相反。

![rotate y](./rotate_y.svg)

类比于xoy平面，那么绕y轴顺时针旋转$`\alpha`$就相当于逆时针旋转$`-\alpha`$,其公式如下：
<!--
[[cos(-alpha),0,-sin(-alpha),0],
 [0         ,1         ,0,0],
 [sin(-alpha),0,cos(-alpha),0],
 [0         ,0         ,0,1]]
((x),(y),(z),(1)) = 
((x'),(y'),(z'),(1))
-->
![](./rotate_y_1.svg)

因为cos 为偶函数，sin为奇函数：
<!--
[[cos(alpha),0,sin(alpha),0],
 [0         ,1         ,0,0],
 [-sin(alpha),0,cos(alpha),0],
 [0         ,0         ,0,1]]
((x),(y),(z),(1)) =
((x'),(y'),(z'),(1))
-->
![](./rotate_y_2.svg)


### 绕x轴旋转

绕x轴旋转的时候，x坐标保持不变。

![](./rotate_x.svg)

为了类比绕z旋转，让我们把x轴翻转一下：

![](./rotate_x_1.svg)

<!--

{ (z'=cos(-alpha)z-sin(-alpha)y),(y'=sin(-alpha)z+cos(-alpha)y):}
rArr 
{ (z'=cos(alpha)z+sin(alpha)y),(y'=-sin(alpha)z+cos(alpha)y):}
-->
![](./rotate_x_2.svg)

写成矩阵的形式：
<!--
[[1,          0,          0,0],
 [0, cos(alpha),-sin(alpha),0],
 [0, sin(alpha), sin(alpha),0],
 [0,          0,          0,1]
]((x),(y),(z),(1)) = ((x'),(y'),(z'),(1))
-->
![](./rotate_x_3.svg)

### 法向量变换矩阵

光线的方向是（1,1,1），仔细观察Fig. 2 会发现光影有点不对。Fig.2 中的小牛的左侧应该没有这么黑。

![](./output.png)

这是因为，在着色的过程中法向量没有做相应的变换。也就是说渲染的模型是变换后的模型，但是用到的每个面的法向量确是变换前的法向量。

那么是否可以对法向量应用同一个变换矩阵呢？

![](./normal_1.svg)

上图中，左边的矩形经过一个剪切变换（shear）得到右边的四边形。左边矩形的右边的法向量（1,0），经过同样的变化后得到（1,1），不再是变换后的四边形右边的法向量了。
<!--
Shear=[[1,0],[1,1]]
-->
![](./normal_2.svg)
<!--
[[1,0],[1,1]]((1),(0))=((1),(1))
-->
![](./normal_3.svg)

实际上，要找到变换后的法向量，只需要用法向量乘以变换的矩阵的逆矩阵的转置矩阵即可。
<!--
n_m = (M^(-1))^(T)n
-->
![](./normal_4.svg)

![](normal.png)
>cargo run --example transform_3d_normal_transform




