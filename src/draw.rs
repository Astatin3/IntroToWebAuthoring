use crate::render::Renderer;

pub fn draw(ctx: &mut Renderer) {
    // // Draw the background
    // ctx.background(0.0, 0.0, 0.0);

    // // Draw the foreground
    // ctx.fill(1.0, 1.0, 1.0);
    // ctx.rect(50.0, 50.0, 100.0, 100.0);
    //

    let (cx, cy) = (ctx.actual_width / 2, ctx.actual_height / 2);

    ctx.circle(cx as i32, cy as i32, 200, (255, 255, 255));
}
