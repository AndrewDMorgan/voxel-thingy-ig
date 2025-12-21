
// Edge function (signed area * 2)
float edge(float2 v0, float2 v1, float2 p) {
    return (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x);
}

bool pointInTriangle(float2 A, float2 B, float2 C, float2 P) {
    float w0 = edge(B, C, P);
    if (w0 < 0.0) return false;
    float w1 = edge(C, A, P);
    if (w1 < 0.0) return false;
    float w2 = edge(A, B, P);
    return w2 >= 0.0;
}

struct Vertex {
    float4 position;
    float4 uv;
};

kernel void ComputeShader (
    constant uint&   pitch             [[ buffer(0 ) ]],  // from sdl2 for padding
    constant uint&   width             [[ buffer(1 ) ]],  //
    constant uint&   height            [[ buffer(2 ) ]],  //

    constant float4& camera_position   [[ buffer(3 ) ]],  // camera position
    constant float4& camera_rotation   [[ buffer(4 ) ]],  // camera rotation

    constant Vertex* vertex_buffer     [[ buffer(5 ) ]],  // triangle data

    constant float4* normals           [[ buffer(6 ) ]],  // number
    constant uint4*  triangles_buffer  [[ buffer(7 ) ]],  // triangle indices
    constant uint*   num_triangles     [[ buffer(8 ) ]],  // number of triangle indices (changed.... no longer that but both tri id's and counts for the binning)

    constant uchar4* texture_buffer    [[ buffer(9 ) ]],  // texture data

    device float*  depth_buffer  [[ buffer(10) ]],
    device uchar*  pixels        [[ buffer(11) ]],

    uint2 gid [[ thread_position_in_grid ]]
) {
    const uint MAX_TRIANGLES_PER_BIN = 64;
    const uint CELL_SIZE = 4;  // seems to be a solid point for this

    if (gid.x >= metal::ceil(float(width) / float(CELL_SIZE)) * CELL_SIZE || gid.y >= metal::ceil(float(height) / float(CELL_SIZE)) * CELL_SIZE) {
        return;
    }

    uint bin_width = metal::ceil(float(width) / float(CELL_SIZE));  // used for the binned triangles array

    uint2 gid_base = gid * CELL_SIZE;
    uint2 gid_max = gid_base + CELL_SIZE;

    float3 sun_direction = metal::normalize(float3(0.5, 1.0, -0.7));

    uint base_index = (gid.x + gid.y * bin_width) * MAX_TRIANGLES_PER_BIN;
    uint tris_in_bin = num_triangles[base_index];
    for (uint bin_id = 0; bin_id < tris_in_bin; bin_id++) {
        uint tri_id = num_triangles[base_index + bin_id + 1];
        uint4 triangle = triangles_buffer[tri_id];
        float3 triangle_normal = normals[triangle.w & 0xFFFF].xyz;
        // I think that's working, but definitely verify bc/ I have no clue (orthographic projection makes this hard to see)
        //if (metal::dot(triangle_normal, camera_rotation.xyz) >= 0.0) { continue; }  // continue;     make this work at some point ig
        float light_intensity = metal::dot(triangle_normal, sun_direction);
        light_intensity = light_intensity * 0.5 + 0.5;
        
        Vertex tri_1 = vertex_buffer[triangle.x];
        Vertex tri_2 = vertex_buffer[triangle.y];
        Vertex tri_3 = vertex_buffer[triangle.z];

        float3 v1 = tri_1.position.xyz;
        float3 v2 = tri_2.position.xyz;
        float3 v3 = tri_3.position.xyz;

        // transforming the vertexes based on rotation and position
        uint maxX = uint(metal::max(v1.x, metal::max(v2.x, v3.x)));
        uint minX = uint(metal::min(v1.x, metal::min(v2.x, v3.x)));
        uint maxY = uint(metal::max(v1.y, metal::max(v2.y, v3.y)));
        uint minY = uint(metal::min(v1.y, metal::min(v2.y, v3.y)));

        if (maxX < gid_base.x || minX > gid_max.x) continue;
        if (maxY < gid_base.y || minY > gid_max.y) continue;

        uchar texture_index = uchar(triangle.w >> 16);

        float area = edge(v1.xy, v2.xy, v3.xy);
        for (uint x = gid_base.x; x < gid_max.x; x++) {
            if (x < minX || x >= width) continue;
            for (uint y = gid_base.y; y < gid_max.y; y++) {
                if (y < minY || y >= height) continue;
                float2 float_coord = float2(float(x), float(y));

                float w0 = edge(v2.xy, v3.xy, float_coord) / area;
                if (w0 < 0) continue;
                float w1 = edge(v3.xy, v1.xy, float_coord) / area;
                if (w1 < 0) continue;
                float w2 = 1.0 - w0 - w1;
                if (w2 < 0) continue;

                float depth = v1.z * w0 + v2.z * w1 + v3.z * w2;
                float uv_u = 16.0 * (w0 * tri_1.uv.x + w1 * tri_2.uv.x + w2 * tri_3.uv.x);
                float uv_v = 16.0 * (w0 * tri_1.uv.y + w1 * tri_2.uv.y + w2 * tri_3.uv.y);

                uint depth_index = x + y * width;
                if (depth >= depth_buffer[depth_index]) continue;
                depth_buffer[depth_index] = depth;

                uint uv_x = uint(uv_u);
                uint uv_y = uint(uv_v);

                uchar3 texture_col = texture_buffer[texture_index * 256 + uv_y * 16 + uv_x].xyz;

                uint pixel_index = x * 3 + (height - y) * pitch;
                pixels[pixel_index + 0] = uchar(texture_col.x * light_intensity);
                pixels[pixel_index + 1] = uchar(texture_col.y * light_intensity);
                pixels[pixel_index + 2] = uchar(texture_col.z * light_intensity);
            }
        }
    }
}

