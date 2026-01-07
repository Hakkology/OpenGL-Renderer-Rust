#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
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
uniform vec3 objectColor;

// Shadow
uniform sampler2D shadowMap;
uniform samplerCube pointShadowMaps[NR_POINT_LIGHTS];
uniform float farPlane;

float calcPointShadow(vec3 fragPos, vec3 lightPos, samplerCube shadowMap, float lightRange) {
    vec3 fragToLight = fragPos - lightPos;
    float currentDepth = length(fragToLight);
    
    // Optimization: Skip shadow if fragment is outside light's influence
    if (currentDepth > lightRange) return 0.0;

    // PCF for Point Shadows (Reduced samples for performance)
    float shadow = 0.0;
    float bias = 0.15; 
    int samples = 8;
    vec3 sampleOffsetDirections[8] = vec3[]
    (
       vec3( 1,  1,  1), vec3( 1, -1,  1), vec3(-1, -1,  1), vec3(-1,  1,  1), 
       vec3( 1,  1, -1), vec3( 1, -1, -1), vec3(-1, -1, -1), vec3(-1,  1, -1)
    );
    
    float viewDistance = length(viewPos - fragPos);
    float diskRadius = (1.0 + (viewDistance / farPlane)) / 50.0;
    
    for(int i = 0; i < samples; ++i) {
        float closestDepth = texture(shadowMap, fragToLight + sampleOffsetDirections[i] * diskRadius).r;
        closestDepth *= farPlane;
        if(currentDepth - bias > closestDepth) {
            shadow += 1.0;
        }
    }
    shadow /= float(samples);
    
    return shadow;
}

float calcShadow(vec4 fragPosLightSpace, vec3 normal, vec3 lightDirNorm) {
    // Perspective divide
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    projCoords = projCoords * 0.5 + 0.5;
    
    // Outside shadow map
    if(projCoords.z > 1.0)
        return 0.0;
    
    float closestDepth = texture(shadowMap, projCoords.xy).r;
    float currentDepth = projCoords.z;
    
    // Bias to prevent shadow acne
    float bias = max(0.05 * (1.0 - dot(normal, lightDirNorm)), 0.005);
    
    // PCF (Percentage Closer Filtering) for softer shadows - Reduced to 3x3 for performance
    float shadow = 0.0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    for(int x = -1; x <= 1; ++x) {
        for(int y = -1; y <= 1; ++y) {
            float pcfDepth = texture(shadowMap, projCoords.xy + vec2(x, y) * texelSize).r;
            shadow += currentDepth - bias > pcfDepth ? 1.0 : 0.0;
        }
    }
    shadow /= 9.0;
    
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
    
    // Apply shadow only to diffuse and specular
    return ambient + (1.0 - shadow) * (diffuse + specular);
}

vec3 calcPointLight(PointLight light, vec3 norm, vec3 viewDir, float shadow) {
    vec3 lightDirNorm = normalize(light.position - FragPos);
    float diff = max(dot(norm, lightDirNorm), 0.0);
    vec3 reflectDir = reflect(-lightDirNorm, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), light.Shininess);
    float distance = length(light.position - FragPos);
    float attenuation = 1.0 / (light.Constant + light.Linear * distance + light.Quadratic * distance * distance);

    vec3 ambient = light.Ambient * light.Color * attenuation;
    vec3 diffuse = light.Diffuse * diff * light.Color * attenuation;
    vec3 specular = light.Specular * spec * light.Color * attenuation;
    
    return ambient + (1.0 - shadow) * (diffuse + specular);
}

// Toggles
uniform int u_UseLighting;
uniform int u_UseShadows;

void main() {
    vec3 result;
    
    if (u_UseLighting == 0) {
        result = vec3(1.0);
    } else {
        vec3 norm = normalize(Normal);
        vec3 viewDir = normalize(viewPos - FragPos);
        vec3 lightDirNorm = normalize(-lightDir);
        
        float shadow = 0.0;
        if (u_UseShadows != 0) {
            shadow = calcShadow(FragPosLightSpace, norm, lightDirNorm);
        }

        result = calcDirLight(norm, viewDir, shadow);
        
        for(int i = 0; i < NR_POINT_LIGHTS; i++) {
            float pShadow = 0.0;
            if (u_UseShadows != 0) {
                // High Optimization: Skip if beyond hard range, and use very few samples
                pShadow = calcPointShadow(FragPos, pointLights[i].position, pointShadowMaps[i], 15.0);
            }
            result += calcPointLight(pointLights[i], norm, viewDir, pShadow);
        }
    }
    
    result *= objectColor;

    FragColor = vec4(result, 1.0);
}
