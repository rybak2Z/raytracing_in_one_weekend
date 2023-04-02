# Introduction

This is my multithreaded 🚀blazingly fast🚀 (not really) Rust implementation of Peter Shirley's 
[Raytracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html#metal) tutorial/book.

Use with `cargo run -r > image.ppm`

Here is an example image that I rendered in only 1h40m 🔥 

![Astonishing world filled with beautiful spheres](example_render.jpg)

# Configuration

`config.toml` provides the following settings:

## Image
Out of the following three settings, you must set two values to what you want and the third one must be omitted. This is most easily done by placing a `#` at the start of the line to comment it out.
- `aspect_ratio`: An array with two elements, e.g. `[16, 9]` for an aspect ratio of 16:9
- `width` in pixels
- `height` in pixels

## Rendering
- `samples_per_pixel`: The number of light rays used to computed the average color of one pixel. This has huge impact on rendering time but if set too low will result in a noisy image.
- `max_child_ray_depth`: How often a light ray can be reflected. There probably won't be even the slightest difference noticeable for numbers above 10.
- `threads`: The total number of threads used (cannot be lower than 1).
- `main_thread_for_render`: By default, if more than one thread is used, then the main thread will *not* participate in rendering. Setting this option to true will make the main thread also render but note that this results in an extremely laggy progress indicator (which is printed to the terminal).
- `update_frequency`: After how many computed pixels the progress indicator should be updated.
- `writing_buffer_capacity`: With more than one thread participating in rendering, new pixels' data will be stored until a continuous line of pixels is complete. This setting determines how many pixels' data can be stored until the storage needs to be increased.

