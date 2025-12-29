struct SolveResult {
    normal: vec3<f32>, // The resulting unit vector (x', y', z')
    z: f32,            // The solved input z
    w: f32,            // The solved input w
    valid: bool        // false if no solution exists (discriminant < 0)
};

// Given x, y, m, find z, w such that m*(wx, wy, z, w) = (x', y', z', 1) such that (x', y', z') has length of 1
fn solve_zw(x: f32, y: f32, m: mat4x4<f32>) -> SolveResult {
    var out: SolveResult;

    // m[0] = col0, m[1] = col1, m[2] = col2, m[3] = col3
    let c0 = m[0]; // m00, m10, m20, m30
    let c1 = m[1]; // m01, m11, m21, m31
    let c2 = m[2]; // m02, m12, m22, m32
    let c3 = m[3]; // m03, m13, m23, m33

    // x' = w*(m00*x + m01*y + m03) + m02*z
    // y' = w*(m10*x + m11*y + m13) + m12*z
    // z' = w*(m20*x + m21*y + m23) + m22*z
    // w' = w*(m30*x + m31*y + m33) + m32*z
    // From w' = 1: (m30*x + m31*y + m33)*w + m32*z = 1
    let Aw = c0.w * x + c1.w * y + c3.w;
    let Bw = c2.w;

    // z = (1-Aw) / Bw
    // If Bw = 0, then w = 1/Aw
    if (abs(Bw) < 1e-6) {
        if (abs(Aw) < 1e-6) {
            out.valid = false;
            return out;
        }

        let w = 1.0 / Aw;

        // We need to find z such that length(C + D*z) = 1 where C = (w*(m00*x + m01*y + m03), w*(m10*x + m11*y + m13), w*(m20*x + m21*y + m23)) and D = (m02, m12, m22)
        let C = (vec3<f32>(c0.xyz) * x + vec3<f32>(c1.xyz) * y + vec3<f32>(c3.xyz)) * w;
        let D = vec3<f32>(c2.xyz);

        // Quadratic: |C + Dz|^2 = 1
        let a = dot(D, D);
        let b = 2.0 * dot(C, D);
        let c = dot(C, C) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if (discriminant < 0.0) {
            out.valid = false; 
            return out;
        }

        // Not sure which root to pick, but this branch never executes (from what I tested), so it hopefully does not matter...
        // It should be + since the camera is inside the sphere so the - intersection is behind the camera
        let z = (-b + sqrt(discriminant)) / (2.0 * a);

        out.z = z;
        out.w = w;
        out.normal = normalize(C + D * z); // normalize ensures precision
        out.valid = true;
        return out;
    }

    // z = (1 - Aw*w) / Bw
    else {
        // Base coefficients for the output vector
        let A_vec = vec3<f32>(c0.xyz) * x + vec3<f32>(c1.xyz) * y + vec3<f32>(c3.xyz);
        let B_vec = vec3<f32>(c2.xyz);

        // Substitute z(w) into output equation to get linear form: U*w + V
        // z = (1/Bw) - (Aw/Bw)*w
        
        // V is the constant part (intercept)
        let V = B_vec * (1.0 / Bw);
        
        // U is the slope part (attached to w)
        let U = A_vec - B_vec * (Aw / Bw);

        // Quadratic: |Uw + V|^2 = 1
        let a = dot(U, U);
        let b = 2.0 * dot(U, V);
        let c = dot(V, V) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if (discriminant < 0.0) {
            out.valid = false;
            return out;
        }

        // Solve w. 
        // Since this case is often connected to perspective projections, it is better to pick w > 0 -> try +sqrt first.
        let w = (-b + sqrt(discriminant)) / (2.0 * a);
        let z = (1.0 - Aw * w) / Bw;

        out.w = w;
        out.z = z;
        out.normal = normalize(U * w + V);
        out.valid = true;
        return out;
    }
}

struct Uniforms {
    inv_mvps: mat4x4<f32>, 
    colour: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var t_cube: texture_cube<f32>;
@group(0) @binding(2) var s_cube: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) ndc: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Trick to generate a full-screen triangle from 3 vertices without buffers
    let uv = vec2<f32>(f32((in_vertex_index << 1u) & 2u), f32(in_vertex_index & 2u));
    
    // Convert 0..2 UV to -1..1 NDC
    out.ndc = uv * 2.0 - 1.0;

    // Z = 1.0 ensures we draw at the far plane
    out.position = vec4<f32>(out.ndc.x, out.ndc.y, 1.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // We are given x, y in device coordinates
    // Projection does this: (x', y', _z', w') = mvps * (x0, y0, z0, 1), (x, y) = (x', y') / w'
    // We have x, y, so (x', y') = (w' * x, w' * y)
    // Then we need to solve inv_mvps * (w' * x, w' * y, z', w') = (x0, y0, z0, 1) where (x0, y0, z0) lies on a sphere of radius 1
    let x = in.ndc.x;
    let y = in.ndc.y;
    let output = solve_zw(x, y, uniforms.inv_mvps);
    if(!output.valid) {
        // Should never happen
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    return textureSample(t_cube, s_cube, output.normal);
}