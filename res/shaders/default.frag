#version 330 core

out vec4 FragColor;

in vec3 crntPos;

in vec3 Normal;
in vec3 color;
in vec2 texCoord;

uniform sampler2D diffuse0;
uniform sampler2D specular0;

uniform vec4 lightColor;
uniform vec3 lightPos;
uniform vec3 camPos;

vec4 pointLight(){

  vec3 lightVec = lightPos - crntPos;
  float dist = length(lightVec);
  float a = 3.0;
  float b = 0.7;
  float inten = 1.0 / (a * dist *dist + b * dist + 1.0);
  
  float ambient = 0.2;


  vec3 normal = normalize(Normal);
  vec3 lightDirection = normalize(lightVec);

  float diffuse = max(dot(normal,lightDirection),0.0);

  float specularLight = 0.5;
  vec3 viewDir = normalize(camPos - crntPos);
   vec3 reflectionDir = reflect(-lightDirection,normal);
   float specAmount = pow(max(dot(viewDir,reflectionDir),0.0),16);
  float specular = specAmount * specularLight;

  return texture(diffuse0,texCoord) * lightColor * (diffuse * inten + ambient) + texture(specular0,texCoord) * specular * inten;
}

vec4 directionalLight(){
 
  float ambient = 0.2;


  vec3 normal = normalize(Normal);
  vec3 lightDirection = normalize(vec3(1.0,1.0,0.0));

  float diffuse = max(dot(normal,lightDirection),0.0);

  float specularLight = 0.5;
  vec3 viewDir = normalize(camPos - crntPos);
   vec3 reflectionDir = reflect(-lightDirection,normal);
   float specAmount = pow(max(dot(viewDir,reflectionDir),0.0),16);
  float specular = specAmount * specularLight;

  return texture(diffuse0,texCoord) * lightColor * (diffuse  + ambient) + texture(specular0,texCoord) * specular ;
}

vec4 spotLight(){
  vec3 lightVec = lightPos - crntPos;
  float outerCone = 0.90;
  float innerCone = 0.95;
  
  float ambient = 0.2;


  vec3 normal = normalize(Normal);
  vec3 lightDirection = normalize(lightVec);

  float diffuse = max(dot(normal,lightDirection),0.0);

  float specularLight = 0.5;
  vec3 viewDir = normalize(camPos - crntPos);
   vec3 reflectionDir = reflect(-lightDirection,normal);
   float specAmount = pow(max(dot(viewDir,reflectionDir),0.0),16);
  float specular = specAmount * specularLight;
  float angle = dot(vec3(0.0,-1.0,0.0),-lightDirection);
  float inten = clamp((angle-outerCone)/(innerCone-outerCone),0.0,1.0);

  return texture(diffuse0,texCoord) * lightColor * (diffuse *inten + ambient) + texture(specular0,texCoord) * specular * inten;
}


void main()
{
  FragColor = pointLight();
}