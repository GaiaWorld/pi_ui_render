
#version 450

layout(location = 0) in vec2 vPosition;

layout(set = 0, binding = 0) uniform ShapeSdfMaterial {
	vec4 uColor;
	vec4 strokeColor;
	vec2 extend;
	vec2 screenSize;
	float radius;
	float strokeSize;
	float blur;
};

float sdfRect(vec2 coord, vec2 size, float r) {
	vec2 d = abs(coord) - size;
	return min(max(d.x, d.y), 0.0) + length(max(d, 0.0)) - r;
}

// http://iquilezles.org/www/articles/ellipsedist/ellipsedist.htm
// https://www.shadertoy.com/view/4sS3zz

float sdfEllipse(vec2 coord, in vec2 radius)
{
	coord = abs(coord); 
	if(coord.x > coord.y) {
		coord = coord.yx;
		radius = radius.yx; 
	}
	
	float l = radius.y * radius.y - radius.x * radius.x;
	
	float m = radius.x * coord.x / l; 
	float n = radius.y * coord.y / l; 
	float m2 = m * m;
	float n2 = n * n;
	
	float c = (m2 + n2 - 1.0) / 3.0; 
	float c3 = c * c * c;

	float q = c3 + m2 * n2 * 2.0;
	float d = c3 + m2 * n2;
	float g = m + m * n2;

	float co;

	if(d < 0.0)
	{
		float h = acos(q / c3) / 3.0;
		float s = cos(h);
		float t = sin(h) * sqrt(3.0);
		float rx = sqrt( -c * (s + t + 2.0) + m2 );
		float ry = sqrt( -c * (s - t + 2.0) + m2 );
		co = (ry + sign(l) * rx + abs(g) / (rx * ry) - m) / 2.0;
	}
	else
	{
		float h = 2.0 * m * n * sqrt(d);
		float s = sign(q + h) * pow( abs(q + h), 1.0 / 3.0 );
		float u = sign(q - h) * pow(abs(q - h), 1.0 / 3.0 );
		float rx = -s - u - c * 4.0 + 2.0 * m2;
		float ry = (s - u) * sqrt(3.0);
		float rm = sqrt(rx * rx + ry * ry);
		co = (ry / sqrt(rm - rx) + 2.0 * g / rm - m) / 2.0;
	}

	float si = sqrt(1.0 - co * co);

	vec2 r = radius * vec2(co, si);
	
	return length(r - coord) * sign(coord.y - r.y);
}

void main(void) {
	// gl_FragCoord的范围是[0, screenSize)，需要变成 [-screenSize/2, screenSize/2)
	vec2 coord = vPosition;

	coord = coord / screenSize;
	vec2 size = extend / screenSize;

	vec4 c = uColor;

	float d;
	#ifdef SDF_RECT
		vec2 rectRadius = radius / screenSize;
		d = sdfRect(coord, 2.0 * size, rectRadius);
	#else
		d = sdfEllipse(coord, 2.0 * size);
	#endif
		float antiBody = 1.0 - smoothstep(-0.002 * blur, 0.002 * blur, d);
		c.a = c.a * antiBody;

	#ifdef STROKE
		vec2 fsStrokeSize = vec2(strokeSize / screenSize);

		#ifdef SDF_RECT
			d = sdfRect(coord, size + fsStrokeSize, rectRadius);
		#else
			d = sdfEllipse(coord, size + fsStrokeSize);
		#endif

		vec4 sc = strokeColor;
		float antiStroke = 1.0 - smoothstep(-0.002 * blur, 0.002 * blur, d);
		sc.a = sc.a * antiStroke;
		c = mix(sc, c, antiBody);
	#endif
	
	c.a = c.a * alpha;
	if (c.a < 0.02) {
		discard;
	}
	
	gl_FragColor = c;
}