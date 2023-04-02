# Introduction

This is my multithreaded 🚀blazingly fast🚀 (not really) Rust implementation of Peter Shirley's 
[Raytracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html#metal) tutorial/book.

Use with `cargo run -r > image.ppm`

~~Here is an example image that I rendered in only 1h40m 🔥~~ Turns out I'm dumb and forgot to run it in release mode. That way it took 28 minutes.

![Astonishing world filled with beautiful spheres](example_render.jpg)

# Configuration

## General configuration

`config.toml` provides the following settings:

### Image
Out of the following three settings, you must set two values to what you want and the third one must be omitted. This is most easily done by placing a `#` at the start of the line to comment it out.
- `aspect_ratio`: An array with two elements, e.g. `[16, 9]` for an aspect ratio of 16:9
- `width` in pixels
- `height` in pixels

### Rendering
- `samples_per_pixel`: The number of light rays used to computed the average color of one pixel. This has huge impact on rendering time but if set too low will result in a noisy image.
- `max_child_ray_depth`: How often a light ray can be reflected. There probably won't be even the slightest difference noticeable for numbers above 10.
- `threads`: The total number of threads used (cannot be lower than 1).
- `main_thread_for_render`: By default, if more than one thread is used, then the main thread will *not* participate in rendering. Setting this option to true will make the main thread also render but note that this results in an extremely laggy progress indicator (which is printed to the terminal).
- `update_frequency`: After how many computed pixels the progress indicator should be updated.
- `writing_buffer_capacity`: With more than one thread participating in rendering, new pixels' data will be stored until a continuous line of pixels is complete. This setting determines how many pixels' data can be stored until the storage needs to be increased.

## Customizing the 3D scene

`default_scene.json` defines the 3D scene to render, i.e. the objects in the scene as well as the camera. It can be customized to build custom scenes.

The file must have the following structure. See `default_scene.json` and the [JSON file format](https://www.json.org/json-de.html) for reference.

- `camera` (object)
    - `look_from` (object) The camera's position
        - `x` (number)
        - `y` (number)
        - `z` (number)
    - `look_at` (object) Where the camera is looking at
        - `x` (number)
        - `y` (number)
        - `z` (number)
    - `view_up_direction` (object) A vector describing what direction is considered "up" for the camera
        - `x` (number)
        - `y` (number)
        - `z` (number)
    - `vertical_fov_degrees` (number) The camera's vertical field of view
    - `aperture` (number) How much light reaches the camera. See [wikipedia article](https://en.wikipedia.org/wiki/Aperture) for more information.
    - `focus_distance` (number or null) The distance from the camera to the plane where everything is in focus. If set to null, then everything is in focus.
    - `focal_length` (number) See [wikipedia article](https://en.wikipedia.org/wiki/Focal_length) for more information.

- `materials` (array) [optional] This array can contain any amount of material definitions that can be referred to by name when defining objects.
    - unnamed (object)
        - `name` (string): This is what is used to refer to the material
        - `type` (string): Possible values: `diffuse`, `metal`, `dialectric` (like glass)
        - `color` (object) Is only needed for materials of type `diffuse` or `metal`
            - `rgb` (array of numbers) An array with three numbers for the red, green, and blue parts of the color. They can either all be number from 0 to 1 or from 0 to 255.
            - `normalized` (boolean) Whether `rgb`'s numbers are between 0 and 1 (true) or between 0 and 255 (false).
        - `fuzziness` (number) Is only required for materials of type `metal`
        - `refractive_index` (number) Is only required for materials of type `dialectric`
    - ...

- `objects` (array) All the objects existing in the world/scene
    - (object)
        - `name` (string) [optional] Does not have any effect. Just for documentation.
        - `coordinates` (array) An array of three numbers for the x, y, and z coordinates
        - `radius` (number) Radius of the sphere
        - `material` (string or object) Can be either a string containing the name of a previously defined material or an object containing a new material definition in the same format as above (though doesn't require a name)
    - ...
