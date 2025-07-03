
struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_cords: vec2<f32>,
  @location(2) normal: vec3<f32>,
  @location(3) instance_transform_0: vec4<f32>,
  @location(4) instance_transform_1: vec4<f32>,
  @location(5) instance_transform_2: vec4<f32>,
  @location(6) instance_transform_3: vec4<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) world_position: vec4<f32>
}


struct Light {
  pos: vec3<f32>,
  color: vec3<f32>,
  view_proj: mat4x4<f32>,
}

@group(0)@binding(0)
var<uniform> light:Light;


@vertex
fn vs_main(v_in: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let instance_matrix = mat4x4<f32>(
        v_in.instance_transform_0,
        v_in.instance_transform_1,
        v_in.instance_transform_2,
        v_in.instance_transform_3,
    );
    output.world_position = instance_matrix * vec4<f32>(v_in.position, 1.0);
    output.clip_position = light.view_proj * instance_matrix * vec4<f32>(v_in.position, 1.0);
    return output;
}


@fragment
fn fs_main(in: VertexOutput) -> @builtin(frag_depth) f32 {
    let depth = length(in.world_position.xyz - light.pos);
    return depth/100.0;
}

