struct VertexInput{
  @location(0) position: vec3<f32>,
  @location(1) tex_cords: vec2<f32>,
  @location(2) instance_transform_0: vec4<f32>,
  @location(3) instance_transform_1: vec4<f32>,
  @location(4) instance_transform_2: vec4<f32>,
  @location(5) instance_transform_3: vec4<f32>,
}


struct VertexOutput{
  @builtin(position) position: vec4<f32>,
  @location(0) tex_cords: vec2<f32>
}


@group(0)
@binding(0)
var<uniform> transform: mat4x4<f32>;


@vertex
fn vs_main(v_in: VertexInput) -> VertexOutput{
  var output: VertexOutput;
  let instance_matrix = mat4x4<f32>(
    v_in.instance_transform_0,
    v_in.instance_transform_1,
    v_in.instance_transform_2,
    v_in.instance_transform_3,
  );
  output.tex_cords = v_in.tex_cords;
  output.position = transform * instance_matrix * vec4<f32>(v_in.position, 1.0);
  return output;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>{
  return textureSample(texture,texture_sampler,in.tex_cords);
}
