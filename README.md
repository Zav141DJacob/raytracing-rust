# RT

## Adjusting objects and scene

You can modify the `config.ron` file to add/remove/edit objects and alter technical details, such as lighting and camera properties.  
'light' field represents brightness and varies from 0 to 100. 'samples' is the amount of pixel samples for antialiasing, should be no less than 1.  

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
