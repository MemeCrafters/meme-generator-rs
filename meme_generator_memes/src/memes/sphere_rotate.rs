use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    tools::{default_sampling_options, local_date, new_surface},
};
use skia_safe::{Data, Image, Paint, RuntimeEffect};

use crate::{options::NoOptions, register_meme};

fn sphere_rotate(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let total_frames = 60;

    let sksl_code = r#"
        uniform shader image;
        uniform float angle;
        uniform float2 canvas_size;
        uniform float2 image_size;

        const float PI = 3.14159265359;

        // Y轴旋转矩阵
        mat3 rotate_y(float a) {
            float s = sin(a);
            float c = cos(a);
            return mat3(
                c, 0, s,
                0, 1, 0,
                -s, 0, c
            );
        }

        // 光线与球体相交检测
        // ro: 光线原点, rd: 光线方向, r: 球体半径
        // 返回 vec2(近交点距离, 远交点距离), 如果不相交则都为-1.0
        vec2 intersect_sphere(vec3 ro, vec3 rd, float r) {
            float b = dot(ro, rd);
            float c = dot(ro, ro) - r * r;
            float h = b * b - c;
            if (h < 0.0) {
                return vec2(-1.0);
            }
            float sqrt_h = sqrt(h);
            return vec2(-b - sqrt_h, -b + sqrt_h);
        }

        // 将球体表面的3D坐标点转换为2D纹理UV坐标 (等距柱状投影)
        vec2 get_sphere_uv(vec3 p) {
            p = normalize(p);
            float u = 0.5 + atan(p.z, p.x) / (2.0 * PI);
            float v = 0.5 + asin(p.y) / PI;
            return vec2(u, v);
        }

        half4 main(vec2 coord) {
            // 将屏幕像素坐标转换为标准化坐标
            vec2 uv = (2.0 * coord - canvas_size.xy) / canvas_size.y;

            // 设置相机 (光线追踪)
            vec3 ro = vec3(0.0, 0.0, 3.5);
            vec3 rd = normalize(vec3(uv, -2.0));

            // 应用Y轴旋转
            mat3 rot = rotate_y(angle);
            ro = rot * ro;
            rd = rot * rd;

            float radius = 1.2;

            // 寻找光线与完整球体的所有交点
            vec2 t = intersect_sphere(ro, rd, radius);
            float t_hit = -1.0;

            // 测试近处的交点
            if (t.x > 0.0) {
                vec3 p1 = ro + rd * t.x;
                if (p1.x >= 0.0) {
                    t_hit = t.x;
                }
            }

            // 如果近处交点无效，则测试远处的交点 (处理看到内壁的情况)
            if (t_hit < 0.0 && t.y > 0.0) {
                vec3 p2 = ro + rd * t.y;
                if (p2.x >= 0.0) {
                    t_hit = t.y;
                }
            }

            // 如果找到了有效的交点，进行着色
            if (t_hit > 0.0) {
                vec3 pos = ro + rd * t_hit;
                vec3 normal = normalize(pos);
                float facing = dot(rd, normal);

                vec2 tex_uv = get_sphere_uv(pos);
                vec2 tex_coords = tex_uv * image_size;
                half4 tex_color = image.eval(tex_coords);

                if (facing < 0.0) {
                    // 外表面: 直接使用纹理颜色
                    return tex_color;
                } else {
                    // 内表面: 将纹理颜色变暗
                    return tex_color * half4(0.7, 0.7, 0.7, 1.0);
                }
            }

            return half4(0.0);
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl_code, None).unwrap();

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let (canvas_w, canvas_h) = (300, 300);
        let mut surface = new_surface((canvas_w, canvas_h));
        let canvas = surface.canvas();

        let angle = i as f32 / total_frames as f32 * std::f32::consts::PI * 2.0;

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "angle" => values.extend(angle.to_le_bytes()),
                "canvas_size" => {
                    values.extend((canvas_w as f32).to_le_bytes());
                    values.extend((canvas_h as f32).to_le_bytes());
                }
                "image_size" => {
                    values.extend((img.width() as f32).to_le_bytes());
                    values.extend((img.height() as f32).to_le_bytes());
                }
                _ => {}
            }
        }
        let uniforms = Data::new_copy(&values);

        let image_shader = img
            .to_shader(None, default_sampling_options(), None)
            .unwrap();
        let shader = effect
            .make_shader(uniforms, &[image_shader.into()], None)
            .unwrap();

        let mut paint = Paint::default();
        paint.set_shader(shader);
        canvas.draw_paint(&paint);

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: total_frames,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "sphere_rotate",
    sphere_rotate,
    min_images = 1,
    max_images = 1,
    keywords = &["球面旋转"],
    date_created = local_date(2025, 7, 6),
    date_modified = local_date(2025, 7, 6),
);
