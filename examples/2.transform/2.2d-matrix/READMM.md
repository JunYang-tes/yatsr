## 运行
```shell
cargo run --example transform_2d_matrix
```
## 线性变换

像这样
$\left[\begin{array}{cc} a & b \\ c & d \\  \end{array}\right]\left(\begin{array}{cc} x \\ y \\  \end{array}\right) = \left(\begin{array}{cc} x' \\ y' \\  \end{array}\right)$ 一个矩阵乘以一个向量，称为对这个向量$\left(\begin{array}{cc} x \\ y \\  \end{array}\right)$ 的线性变化。

### 缩放
$$
\begin{array}{cc} x^{\prime} = s_x \cdot x \\ y^{\prime} = s_y \cdot y \\  \end{array}
$$
写成矩阵的形式：
$$
\left[\begin{array}{cc} S_x & 0 \\ 0 & S_y \\  \end{array}\right]\left(\begin{array}{cc} x \\ y \\  \end{array}\right) = \left(\begin{array}{cc} x' \\ y' \\  \end{array}\right)
$$

### Shear

$$\displaystyle{\left\lbrace\begin{matrix}{x}'={x}+{s}_{{x}}{y}\\{y}'={s}_{{y}}{x}+{y}\end{matrix}\right.}$$

写成矩阵的形式：
$$
\left[\begin{array}{cc} 1 & s_x \\ s_y & 1 \\  \end{array}\right]\left(\begin{array}{cc} x \\ y \\  \end{array}\right) = \left(\begin{array}{cc} x' \\ y' \\  \end{array}\right)
$$

### 绕（0,0）逆时针旋转

$$\displaystyle{\left\lbrace\begin{matrix}{x}'= \cos{{\left(\alpha\right)}}{x}- \sin{{\left(\alpha\right)}}{y}\\{y}'= \sin{{\left(\alpha\right)}}{x}+ \cos{{\left(\alpha\right)}}{y}\end{matrix}\right.}$$

写成矩阵的形式：
$$\displaystyle{\left[\begin{matrix} \cos{{\left(\alpha\right)}}&- \sin{{\left(\alpha\right)}}\\ \sin{{\left(\alpha\right)}}& \cos{{\left(\alpha\right)}}\end{matrix}\right]}{\left(\begin{matrix}{x}\\{y}\end{matrix}\right)}={\left(\begin{matrix}{x}'\\{y}'\end{matrix}\right)}$$

### 平移
$$\displaystyle{\left\lbrace\begin{matrix}{x}'={x}+{\left.{d}{x}\right.}\\{y}'={y}+{\left.{d}{y}\right.}\end{matrix}\right.}$$

糟糕，没法写成矩阵的形式，借助三阶：
$$\displaystyle{\left[\begin{matrix}{1}&{0}&{\left.{d}{x}\right.}\\{0}&{1}&{\left.{d}{x}\right.}\\{0}&{0}&{1}\end{matrix}\right]}{\left(\begin{matrix}{x}\\{y}\\{1}\end{matrix}\right)}={\left(\begin{matrix}{x}'\\{y}'\\{1}\end{matrix}\right)}$$
这就是仿射变化（affine），（x',y',1)被称为齐次坐标，(x,y)=(x',y',1)=(x'/z,y'/z,z/z),点-点=向量，(x,y,1)-(x',y',1) = (x-x',y-y',0),因此,齐次坐标第三个分量为0表示向量。（x,y,1)+(x',y',1)=(x+x',y+y',2)=((x+x')/2,(y+y')/2,1),因此点+点=中点.

## 变换的组合

在做一系列的变化的情况下，比如对一个列向量v先做一个缩放（S)，即$\displaystyle{v}'={S}{v}$;再做一个平移（T），既$\displaystyle{v}{''}={T}{v}'={T}{\left({S}{v}\right)}$, 因为矩阵乘法具有结合性，所以：$\displaystyle{v}{''}={T}{\left({S}{v}\right)}={\left({T}{S}\right)}{v}$。这说明：
1. 对于一系列的变化，我们可以把这一系列变换的矩阵预先乘起来
2. $\displaystyle{\left({T}_{{1}}{T}_{{2}}{T}_{{3}}\ldots{T}_{{n}}\right)}{v}$ 表示先对${v}$ 做${T}_{{n}}$ 变换，再做${T}_{{n-1}}$变换..., 最后做${T}_{{1}}$ 变换

## 逆变化

从几何的角度很容易找到这些变换的逆变换。因为放大了的，缩小就是。平移了的是，向反方向再平移就是。因此如果用$\displaystyle{S}{\left({s}_{{x}},{s}_{{y}}\right)}$,来表示缩放变换，那么缩放变换的逆变换为：
$\displaystyle{S}{\left({s}_{{x}},{s}_{{y}}\right)}^{{-{1}}}={S}{\left(\frac{1}{{s}_{{x}}},\frac{1}{{s}_{{y}}}\right)}$。用$\displaystyle{T}{\left({\left.{d}{x}\right.},{\left.{d}{y}\right.}\right)}$来标识平移，那么平移的逆变换为：$\displaystyle{T}{\left(-{\left.{d}{x}\right.},-{\left.{d}{y}\right.}\right)}$。用$\displaystyle{R}{\left(\alpha\right)}$表示绕（0,0）逆时针旋转，那么其逆变换为：$\displaystyle{R}{\left(-\alpha\right)}$

那么复合的变换的逆变化呢？从几何的角度也很容易得到其逆，比如，先旋转再平移的逆变换自然是先平移回去再旋转回去。用矩阵乘表示：
$$
\begin{align}
v'=TSv \\
(TS)^{-1}v'=(TS)^{-1}TSv \\
S^{-1}T^{-1}v'=v
\end{align}
$$

另外，旋转矩阵是正交矩阵，其逆是其转置。
