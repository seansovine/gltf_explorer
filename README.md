# glTF Explorer

This program is for exploring the contents of a glTF file using the Rust gltf crate.

Currently, it will read in a .glb file and write the contents of the file's glTF
JSON data to the console as a JSON string. This is interestingly the most reliable
and simple way I've found to do this.

## Next

Next, we will work on using this to extract the actual scene data that we need.

To help with this we can look at the code of Willems' C++ renderer, and try to
translate what we find to Rust. This is particularly useful because he handles
the PBR texture information and uses it in his shaders.

We will also look at how to construct the correct transformations for each node
and mesh in the scene. Willems' renderer will be a useful reference here too.

Once we have built a library to do these things, we can then use this in our
wgpu_grapher project to more completely load and render glTF models.
