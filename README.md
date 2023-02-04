# RT

## Adjusting objects and scene

You can modify the `config.ron` file to add/remove/edit objects and alter technical details, such as lighting and camera properties.  

### Main settings
*light* - the value should be between 0-100, lower value will create darker picture

*samples* - is the amount of pixel samples for antialiasing, should be positive number. The higher value - the better picture quality you will get (but also will take more time to render the picture)

*width* and *height* - resolution of output image 

### Materials
There are three kinds of materials available: [Lambertian](https://en.wikipedia.org/wiki/Lambertian_reflectance), Metal, and Dielectric. Their formats are as follows:  
```
material: Lambertian(
    albedo: Vec3(0.4, 0.4, 1.0),   // R, G, B; 0.0-1.0
)
```
```
material: Metal(
    albedo: Vec3(0.4, 0.4, 1.0),   // R, G, B; 0.0-1.0
)
```
```
material: Dielectric(
    ref_idx: 0.5,                  // Refractive index; see https://en.wikipedia.org/wiki/Refractive_index
)
```

### Figures
If you want to add new figure, just add it to the ***world***.

#### Example - sphere
```
{
    "Sphere": (
    center: Vec3(2.0, 0.0, -1.0),
    radius: 0.5,
    material: Lambertian( albedo: Vec3(0.2, 0.2, 1.0) ) 
    )
},
```
For other figures, please refer to [config.ron](config.ron)

### Camera
Camera has some interesting options as well.
```
camera: (
        look_from: Vec3(1.0, 3.0, 4.0),
        look_at: Vec3(0.0, 0.0, -1.0),
        vup: Vec3(0.0, 1.0, 0.0),
        vfov: 45.0,
        aperture: 0.1,
    )
```
*look_from* and *look_at* are describing from which point and to which direction camera is looking. Parameter *vfov* is setting camra view angle - this can be used as zoom in and out - for 90.0 it will be zoomed out an at 30.0-45.0 degrees it will be zoomed in to the objects. *aperture* is used for focus depth - the higher number, the less onjects will be in focus - this creates blur effect.

## Prerequisites

Rust nightly and [FFmpeg](https://www.ffmpeg.org/) are required.  
Rust nightly is installed like so:  
``
rustup install nightly
``  
or  
``
rustup default nightly
``

## Running the program and rendering

``
make run
``

This will create both ``example.ppm`` and ``example.png`` displaying your scene.
