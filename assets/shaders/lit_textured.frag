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

// Point Light
uniform vec3 pointLightPos;
uniform vec3 pointColor;
uniform float pointAmbient;
uniform float pointDiffuse;
uniform float pointSpecular;
uniform float pointShininess;
uniform float pointConstant;
uniform float pointLinear;
uniform float pointQuadratic;

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
    
    return ambient + (1.0 - shadow) * (diffuse + specular);
}

vec3 calcPointLight(vec3 norm, vec3 viewDir) {
    vec3 lightDirNorm = normalize(pointLightPos - FragPos);
    float diff = max(dot(norm, lightDirNorm), 0.0);
    vec3 reflectDir = reflect(-lightDirNorm, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), pointShininess);
    float distance = length(pointLightPos - FragPos);
    float attenuation = 1.0 / (pointConstant + pointLinear * distance + pointQuadratic * distance * distance);

    vec3 ambient = pointAmbient * pointColor * attenuation;
    vec3 diffuse = pointDiffuse * diff * pointColor * attenuation;
    vec3 specular = pointSpecular * spec * pointColor * attenuation;
    return ambient + diffuse + specular;
}

void main() {
    vec4 texColor = texture(u_Texture, TexCoord);
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 lightDirNorm = normalize(-lightDir);
    
    float shadow = calcShadow(FragPosLightSpace, norm, lightDirNorm);

    vec3 result = calcDirLight(norm, viewDir, shadow);
    result += calcPointLight(norm, viewDir);
    result *= texColor.rgb;

    FragColor = vec4(result, texColor.a);
}
