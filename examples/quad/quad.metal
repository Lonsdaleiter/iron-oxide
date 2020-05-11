#include <metal_stdlib>

using namespace metal;

struct ToVertex {
	float4 position;
	float4 colour;
};

struct ToFragment {
    float4 position [[position]];
    float4 colour;
};

vertex ToFragment quad_v(device ToVertex* vertexArray [[ buffer(0) ]],
                         unsigned int vid [[ vertex_id ]])
{
    ToFragment out;
    out.position = vertexArray[vid].position;
    out.colour = vertexArray[vid].colour;
    return out;
}

fragment float4 quad_f(ToFragment in [[stage_in]])
{
    return in.colour;
}
