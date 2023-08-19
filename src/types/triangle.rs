/// 三角形を描画するための構造体
pub struct Triangle {
    /// * `points`  - 各頂点の座標を(x, y)の形式で受け取る。\[三角形の上、三角形の左下、三角形の右下\]の順番になる。
    pub points: [(f64, f64); 3],
    /// * `color`  - 三角形を描画する際の色を受け取る。(red, green, blue)の形式。
    pub color: (u8, u8, u8),
}

impl Triangle {
    /// 三角形を描画する処理
    /// * `context`  - Canvas要素
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        let color_str = format!("rgb({}, {}, {})", self.color.0, self.color.1, self.color.2);
        context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

        let [top, left, right] = self.points;

        context.move_to(top.0, top.1);
        context.begin_path();
        context.line_to(left.0, left.1);
        context.line_to(right.0, right.1);
        context.line_to(top.0, top.1);
        context.close_path();
        context.stroke();
        context.fill();
    }
}

