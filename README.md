# Ray Tracer
Simple, parallized ray tracer written in rust.

## Example Renders
Without Gamma Correction

![lambertian_bp](output/lambertian_blinn_phong.png)


With Gamma Correction

![lambertian_bp_cc](output/lambertian_blinn_phong_cc.png)

## Task List 
### Cameras
- [x] Positionable Camera
- [x] Perspective
- [ ] Orthographic 

### Geometries
- [x] Ray
- [x] Sphere
- [x] Plane
- [ ] Cone
- [ ] Cylinder
- [ ] Triangle Meshes
- [ ] Constructive Solid Geometries

### Illumination
- [x] Ambient light
- [x] Point light
- [x] Lambertian Diffuse
- [x] Blinn-Phong Specular
- [ ] Light Scatter and Reflectance
- [ ] Dialectrics

### Graphics
- [ ] Texture Mapping
- [ ] Depth of field 

### Png Export
- [x] RGB
- [x] Grayscale
- [x] Gamma correction

### Scenes
- [ ] Blender multi-object scene import

### Anti Aliasing
- [x] multiple samples per pixel

### Performance
- [ ] Scale to N cores



