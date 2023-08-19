use rand::prelude::*;

/// 三角形を描画するための構造体
pub struct Triangle {
    /// * `points`  - 各頂点の座標を(x, y)の形式で受け取る。\[三角形の上、三角形の左下、三角形の右下\]の順番になる。
    pub points: [(f64, f64); 3],
    /// * `color`  - 三角形を描画する際の色を受け取る。(red, green, blue)の形式。
    pub color: (u8, u8, u8),
}

impl Triangle {
    /// シェルピンスキーの三角形を描画する
    /// * `context` - canvas要素を受け取る
    /// * `depth`  - 再帰の深さ
    pub fn sierpinski(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        depth: u8,
    ) {
        self.draw(context);

        let depth = depth - 1;
        let [top, left, right] = self.points;

        if depth > 0 {
            let mut rng = thread_rng();

            let next_color = (
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            );

            let left_middle = Self::midpoint(top, left);
            let right_middle = Self::midpoint(top, right);
            let bottom_middle = Self::midpoint(left, right);

            let top_triangle = Triangle { points: [top, left_middle, right_middle], color: next_color };
            let left_triangle = Triangle { points: [left_middle, left, bottom_middle], color: next_color };
            let right_triangle = Triangle { points: [right_middle, bottom_middle, right], color: next_color };

            top_triangle.sierpinski(
                &context,
                depth,
            );
            left_triangle.sierpinski(
                &context,
                depth,
            );
            right_triangle.sierpinski(
                &context,
                depth,
            );
        }
    }

    /// 小さい三角形を描画するために、新たな座標を計算して返す処理
    fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64) {
        ((point_1.0 + point_2.0) / 2.0, (point_1.1 + point_2.1) / 2.0)
    }

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

