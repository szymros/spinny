struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_cords: vec2<f32>,
  @location(2) normals: vec3<f32>,
}


struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec3<f32>
}


struct Camera {
  view_matrix: mat4x4<f32>,
  pos: vec4<f32>
}

@group(0)
@binding(0)
var<uniform> camera: Camera;


struct Light {
  position: vec3<f32>,
  color: vec3<f32>
}


@group(0)
@binding(1)
var<uniform> light: Light;


@vertex
fn vs_main(v_in: VertexInput) -> VertexOutput {
    let scale = 0.25;
    var output: VertexOutput;
    let world_pos = v_in.position * scale + light.position;
    output.position = camera.view_matrix * vec4<f32>(world_pos, 1.0);
    output.color = light.color;
    return output;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
