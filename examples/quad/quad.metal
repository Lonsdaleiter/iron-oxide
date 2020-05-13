#include <metal_stdlib>

using namespace metal;

struct ToQuadVertex {
    float2 position;
	packed_float4 colour;
};

struct ToQuadFragment {
    float4 position [[position]];
    float4 colour;
};

vertex ToQuadFragment quad_v(device ToQuadVertex* vertexArray [[ buffer(0) ]],
                         unsigned int vid [[ vertex_id ]])
{
    ToQuadFragment out;
    out.position = float4(vertexArray[vid].position, 0.0, 1.0);
    out.colour = vertexArray[vid].colour;
    return out;
}

fragment float4 quad_f(ToQuadFragment in [[stage_in]])
{
    return in.colour;
}
