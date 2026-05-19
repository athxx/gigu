use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.ViewBase

    mod.widgets.BottomNavBar = ViewBase{
        clip_x: false
        clip_y: false
        show_bg: true
        flow: Right
        align: Align{y: 0.5}
        width: Fill
        height: 64

        draw_bg +: {
            color: instance(#xfafaf7)
            color_2: instance(vec4(-1))
            color_dither: uniform(1.0)
            gradient_fill_horizontal: uniform(0.0)

            border_size: uniform(0.0)
            border_color: instance(#0000)
            border_radius: uniform(0.0)

            tint_alpha: instance(1.0)
            glass_amount: instance(0.0)
            specular_strength: instance(0.35)
            noise_strength: instance(0.02)

            shadow_color: instance(#0000)
            shadow_radius: uniform(0.0)
            shadow_offset: uniform(vec2(0))

            rect_size2: varying(vec2(0))
            rect_size3: varying(vec2(0))
            rect_pos2: varying(vec2(0))
            rect_shift: varying(vec2(0))
            sdf_rect_pos: varying(vec2(0))
            sdf_rect_size: varying(vec2(0))

            vertex: fn() {
                let min_offset = min(self.shadow_offset vec2(0))
                self.rect_size2 = self.rect_size + 2.0*vec2(self.shadow_radius)
                self.rect_size3 = self.rect_size2 + abs(self.shadow_offset)
                self.rect_pos2 = self.rect_pos - vec2(self.shadow_radius) + min_offset
                self.sdf_rect_size = self.rect_size2 - vec2(self.shadow_radius * 2.0 + self.border_size * 2.0)
                self.sdf_rect_pos = -min_offset + vec2(self.border_size + self.shadow_radius)
                self.rect_shift = -min_offset

                return self.clip_and_transform_vertex(self.rect_pos2 self.rect_size3)
            }

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size3)

                let mut base_rgb = self.color.rgb
                if self.color_2.x > -0.5 {
                    let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                    let dir = if self.gradient_fill_horizontal > 0.5 self.pos.x else self.pos.y
                    base_rgb = mix(self.color.rgb self.color_2.rgb dir + dither)
                }

                let glass_tone = vec3(0.86 0.9 0.96)
                let edge_uv = abs(self.pos * 2.0 - 1.0)
                let edge_gradient = clamp((edge_uv.x + edge_uv.y) * 0.5 0.0 1.0)
                let highlight = self.specular_strength * (0.65 * edge_gradient + 0.35 * (1.0 - self.pos.y))
                let noise = (Math.random_2d(self.pos * self.rect_size) - 0.5) * self.noise_strength

                let glass_rgb = mix(base_rgb glass_tone 0.45) + highlight + noise
                let final_rgb = mix(base_rgb glass_rgb clamp(self.glass_amount 0.0 1.0))
                let final_alpha = self.color.a * self.tint_alpha
                let fill_color = vec4(final_rgb final_alpha)

                sdf.box(
                    self.sdf_rect_pos.x
                    self.sdf_rect_pos.y
                    self.sdf_rect_size.x
                    self.sdf_rect_size.y
                    max(1.0 self.border_radius)
                )
                if sdf.shape > -1.0 {
                    let m = self.shadow_radius
                    let o = self.shadow_offset + self.rect_shift
                    let v = GaussShadow.rounded_box_shadow(vec2(m) + o self.rect_size2+o self.pos * (self.rect_size3+vec2(m)) self.shadow_radius*0.5 self.border_radius*2.0)
                    sdf.clear(self.shadow_color*v)
                }

                sdf.fill_keep(fill_color)
                if self.border_size > 0.0 {
                    sdf.stroke(self.border_color self.border_size)
                }
                return sdf.result
            }
        }
    }
}
