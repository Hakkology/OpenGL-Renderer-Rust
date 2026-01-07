#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoord;
in vec4 FragPosLightSpace;

// Directional Light
uniform vec3 lightDir;
uniform vec3 lightColor;
uniform float ambientStrength;
uniform float diffuseStrength;
uniform float specularStrength;
uniform float shininess;

// Point Lights
struct PointLight {
    vec3 position;
    vec3 Color;
    float Ambient;
    float Diffuse;
    float Specular;
    float Shininess;
    float Constant;
    float Linear;
    float Quadratic;
};
#define NR_POINT_LIGHTS 4
uniform PointLight pointLights[NR_POINT_LIGHTS];

uniform vec3 viewPos;
uniform sampler2D u_Texture;

// Shadow
uniform sampler2D shadowMap;

float calcShadow(vec4 fragPosLightSpace, vec3 normal, vec3 lightDirNorm) {
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    projCoords = projCoords * 0.5 + 0.5;
    
    if(projCoords.z > 1.0)
        return 0.0;
    
    float closestDepth = texture(shadowMap, projCoords.xy).r;
    float currentDepth = projCoords.z;
    float bias = max(0.05 * (1.0 - dot(normal, lightDirNorm)), 0.005);
    
    // PCF
    float shadow = 0.0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    for(int x = -2; x <= 2; ++x) {
        for(int y = -2; y <= 2; ++y) {
            float pcfDepth = texture(shadowMap, projCoords.xy + vec2(x, y) * texelSize).r;
            shadow += currentDepth - bias > pcfDepth ? 1.0 : 0.0;
        }
    }
    shadow /= 25.0;
    
    return shadow;
}

vec3 calcDirLight(vec3 norm, vec3 viewDir, float shadow) {
    vec3 lightDirNorm = normalize(-lightDir);
    float diff = max(dot(norm, lightDirNorm), 0.0);
    vec3 reflectDir = reflect(-lightDirNorm, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    
    vec3 ambient = ambientStrength * lightColor;
    vec3 diffuse = diffuseStrength * diff * lightColor;
    vec3 specular = specularStrength * spec * lightColor;
    
    return ambient + (1.0 - shadow) * (diffuse + specular);
}

vec3 calcPointLight(PointLight light, vec3 norm, vec3 viewDir) {
    vec3 lightDirNorm = normalize(light.position - FragPos);
    float diff = max(dot(norm, lightDirNorm), 0.0);
    vec3 reflectDir = reflect(-lightDirNorm, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), light.Shininess);
    float distance = length(light.position - FragPos);
    float attenuation = 1.0 / (light.Constant + light.Linear * distance + light.Quadratic * distance * distance);

    vec3 ambient = light.Ambient * light.Color * attenuation;
    vec3 diffuse = light.Diffuse * diff * light.Color * attenuation;
    vec3 specular = light.Specular * spec * light.Color * attenuation;
    return ambient + diffuse + specular;
}

// Toggles
uniform int u_UseLighting;
uniform int u_UseShadows;

uniform int u_IsRepeated;
uniform vec2 u_UVScale;

void main() {
    vec2 coords = TexCoord;
    if (u_IsRepeated != 0) {
        coords.x *= u_UVScale.x;
        coords.y *= u_UVScale.y;
    }
    
    vec4 texColor = texture(u_Texture, coords);
    vec3 result;

    if (u_UseLighting == 0) {
        result = vec3(1.0); // Unlit logic: just pure white multiplier (returns texture color)
    } else {
        vec3 norm = normalize(Normal);
        vec3 viewDir = normalize(viewPos - FragPos);
        vec3 lightDirNorm = normalize(-lightDir);
        
        float shadow = 0.0;
        if (u_UseShadows != 0) {
            shadow = calcShadow(FragPosLightSpace, norm, lightDirNorm);
        }

        result = calcDirLight(norm, viewDir, shadow);
        // result += calcPointLight(norm, viewDir);
        for(int i = 0; i < NR_POINT_LIGHTS; i++)
            result += calcPointLight(pointLights[i], norm, viewDir);
    }
    
    result *= texColor.rgb;

    FragColor = vec4(result, texColor.a);
}
