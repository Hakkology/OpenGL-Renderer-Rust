#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec4 FragPosLightSpace;

// Directional Light Properties
uniform vec3 lightDir;
uniform vec3 lightColor;
uniform float ambientStrength;
uniform float diffuseStrength;
uniform float specularStrength;
uniform float shininess;

// Point Light Structure
struct PointLight {
    vec3 position;
    vec3 Color;
    float Ambient;
    float Diffuse;
    float Specular;
    float Shininess; // Use light shininess if specialized, else material
    float Constant;
    float Linear;
    float Quadratic;
};
#define NR_POINT_LIGHTS 4
uniform PointLight pointLights[NR_POINT_LIGHTS];

// Spot Light Structure
struct SpotLight {
    vec3 position;
    vec3 direction;
    float CutOff;
    float OuterCutOff;
  
    float Constant;
    float Linear;
    float Quadratic;
  
    vec3 Color;
    float Ambient;
    float Diffuse;
    float Specular;
};
#define NR_SPOT_LIGHTS 4
uniform SpotLight spotLights[NR_SPOT_LIGHTS];

uniform vec3 viewPos;
uniform vec3 objectColor;

// Shadow Maps
uniform sampler2D shadowMap;
uniform samplerCube pointShadowMaps[NR_POINT_LIGHTS];
uniform float farPlane;
uniform int nrPointLights;
uniform int nrSpotLights;

// Calculate Point Shadow (with PCF)
float calcPointShadow(vec3 fragPos, vec3 lightPos, samplerCube shadowMap, float lightRange) {
    vec3 fragToLight = fragPos - lightPos;
    float currentDepth = length(fragToLight);
    
    // Skip if out of range
    if (currentDepth > lightRange) return 0.0;

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
    
    return shadow / float(samples);
}

// Calculate Directional Shadow
float calcShadow(vec4 fragPosLightSpace, vec3 normal, vec3 lightDirNorm) {
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    projCoords = projCoords * 0.5 + 0.5;
    
    if(projCoords.z > 1.0) return 0.0;
    if(projCoords.x < 0.0 || projCoords.x > 1.0 || projCoords.y < 0.0 || projCoords.y > 1.0) return 0.0;
    
    float currentDepth = projCoords.z;
    float bias = max(0.05 * (1.0 - dot(normal, lightDirNorm)), 0.005);
    
    // PCF (3x3 sampling)
    float shadow = 0.0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    for(int x = -1; x <= 1; ++x) {
        for(int y = -1; y <= 1; ++y) {
            float pcfDepth = texture(shadowMap, projCoords.xy + vec2(x, y) * texelSize).r;
            shadow += currentDepth - bias > pcfDepth ? 1.0 : 0.0;
        }
    }
    
    return shadow / 9.0;
}

// Directional Light Calculation
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

// Point Light Calculation
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

// Spot Light Calculation
vec3 calcSpotLight(SpotLight light, vec3 norm, vec3 fragPos, vec3 viewDir) {
    vec3 lightDir = normalize(light.position - fragPos);
    
    // Diffuse shading
    float diff = max(dot(norm, lightDir), 0.0);
    
    // Specular shading
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    
    // Attenuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.Constant + light.Linear * distance + light.Quadratic * (distance * distance));    
    
    // Spotlight intensity
    float theta = dot(lightDir, normalize(-light.direction)); 
    float epsilon = light.CutOff - light.OuterCutOff;
    float intensity = clamp((theta - light.OuterCutOff) / epsilon, 0.0, 1.0);
    
    // Combine
    vec3 ambient = light.Ambient * light.Color * attenuation; // Ambient always present but attenuated
    vec3 diffuse = light.Diffuse * diff * light.Color * intensity * attenuation;
    vec3 specular = light.Specular * spec * light.Color * intensity * attenuation;
    
    return ambient + diffuse + specular;
}

uniform int u_UseLighting;
uniform int u_UseShadows;

void main() {
    vec3 result;
    
    if (u_UseLighting == 0) {
        result = vec3(1.0);
    } else {
        vec3 norm = normalize(Normal);
        vec3 viewDir = normalize(viewPos - FragPos);
        
        // Two-sided lighting
        if (dot(norm, viewDir) < 0.0) {
            norm = -norm;
        }
        vec3 lightDirNorm = normalize(-lightDir);
        
        // Directional Shadow
        float NdotL = dot(norm, lightDirNorm);
        float shadow = 0.0;
        if (u_UseShadows != 0 && NdotL > 0.0) {
            shadow = calcShadow(FragPosLightSpace, norm, lightDirNorm);
        }

        // Directional Light
        result = calcDirLight(norm, viewDir, shadow);
        


        // Point Lights
        for(int i = 0; i < nrPointLights; i++) {
            float pShadow = 0.0;
            if (u_UseShadows != 0) {
                vec3 lightToFrag = normalize(FragPos - pointLights[i].position);
                if (dot(norm, -lightToFrag) > 0.0) {
                    pShadow = calcPointShadow(FragPos, pointLights[i].position, pointShadowMaps[i], 15.0);
                }
            }
            result += calcPointLight(pointLights[i], norm, viewDir, pShadow);
        }



        // Spot Lights
        for(int i = 0; i < nrSpotLights; i++) {
            result += calcSpotLight(spotLights[i], norm, FragPos, viewDir);
        }
    }
    
    result *= objectColor;
    FragColor = vec4(result, 1.0);
}
