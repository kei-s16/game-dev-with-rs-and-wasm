/// 三角形を描画するための構造体
pub struct Triangle {
    /// * `points`  - 各頂点の座標を(x, y)の形式で受け取る。\[三角形の上、三角形の左下、三角形の右下\]の順番になる。
    pub points: [(f64, f64); 3],
    /// * `color`  - 三角形を描画する際の色を受け取る。(red, green, blue)の形式。
    pub color: (u8, u8, u8),
}
