struct Camera {
    view_projection: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) texture_coordinate: vec2<f32>,
    @location(3) model_0: vec4<f32>,
    @location(4) model_1: vec4<f32>,
    @location(5) model_2: vec4<f32>,
    @location(6) model_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
}

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let model = mat4x4<f32>(input.model_0, input.model_1, input.model_2, input.model_3);
    output.position = camera.view_projection * (model * vec4<f32>(input.position, 1.0));
    let normal_0 = cross(input.model_1.xyz, input.model_2.xyz);
    let normal_1 = cross(input.model_2.xyz, input.model_0.xyz);
    let normal_2 = cross(input.model_0.xyz, input.model_1.xyz);
    let orientation = sign(dot(input.model_0.xyz, normal_0));
    output.normal = (mat3x3<f32>(normal_0, normal_1, normal_2) * input.normal) * orientation;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.normal);
    let light_direction = vec3<f32>(0.2981424, 0.5962848, 0.7453560);
    let light = 0.25 + max(dot(normal, light_direction), 0.0) * 0.75;
    let color = abs(normal) * 0.55 + vec3<f32>(0.25);

    return vec4<f32>(color * light, 1.0);
}
