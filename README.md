# About

A simple library to generate 2-dimensional Signed Distance Fields (SDF) for font rendering.  
Please note that this is my personal playground to learn the Rust programming language, so don't expect any highly 
sophisticated code samples here. I will try to improve this library over time, however right now it is not recommended
to use it for anything and especially not in production.  😉

## Examples

| Input | Output |  
| --- | --- |
| <img alt="input image 1" src="assets/example_1_rgba_512x512.png" width="256" height="256" /> | <img alt="output image 1" src="output/odf_example_1_512x512.png" width="256" height="256" /> |
| <img alt="input image 2" src="assets/example_2_rgba_512x512.png" width="256" height="256" /> | <img alt="output image 2" src="output/odf_example_2_512x512.png" width="256" height="256" /> |
| <img alt="input image 3" src="assets/example_3_rgba_512x512.png" width="256" height="256" /> | <img alt="output image 3" src="output/odf_example_3_512x512.png" width="256" height="256" /> |
| <img alt="input image 4" src="assets/example_4_rgba_512x512.png" width="256" height="256" /> | <img alt="output image 4" src="output/odf_example_4_512x512.png" width="256" height="256" /> |


## Links about SDFs and there usage (for example in text rendering) 

1) [Drawing Text with Signed Distance Fields in Mapbox GL](https://blog.mapbox.com/drawing-text-with-signed-distance-fields-in-mapbox-gl-b0933af6f817)  
Interesting blog post about MapBox and how they SDF to render the labels on their maps - 
including some source code for a FragmentShader

1) [Signed Distance Fields](https://github.com/chriscummings100/signeddistancefields/blob/master/Assets/SignedDistanceFields/SignedDistanceFieldGenerator.cs)  
C# code repository of a SDF generator with different generation algorithms 
(8PSSDT sweep, brute force Eikonal etc.)

1) [Shader Fun](https://shaderfun.com/)  
Blogpost series abouts SDFs

1) [Distance Field Fonts](https://github.com/libgdx/libgdx/wiki/Distance-field-fonts)  
Nixe explanation about the usage of SDF fonts in libGDX  

1) [Ronjas Shader Tutorials](https://www.ronja-tutorials.com/2018/11/10/2d-sdf-basics.html)
2D Signed Distance Field Basics - blog with some posts about SDFs

## Fluent syntax

##### Current version
```rust
let gen = DistanceGenerator::new()
            .input("/path")
            .output("/path")
            .generate();
```

##### Future versions

```rust
    let gen = DistanceGenerator()
        .withConfiguration(GeratorConfiguration())
        .fromInput(ImageFileInput("/path"))
        .toOutput(FileOutput("/path")) // also ImageBufferOutput
        .generate();
```

## Input
From ImageBuffer  
From File (File -> ImageBuffer)  
From TTF Font File  
Additional Transformers would be nice too:
Rotate, Flip, B/W, ChannelMatrix (Re-Map Channels), Threshold,
scale (up/down) <- very important ... inc. linear/bilinear/triliniear interpolation
Bits per channel Kanal (1/8/16/32/64) (Source format) - 
But how do we calculate the output? ->  B/W vs. Greyscale

## Output  
ImageBuffer
File (just a special transformation of an ImageBuffers. Which channels do we use?   
8/16/32 Bit?  
Int or Float?  
Even here we have the options to do multiple transformations (scale, rotate, flip)  


## Configuration  
Used Algorithm (Naive, 8SED, Dead Reckoning, etc.)

In theory it should be possible to apply this to TTF Fonts. For that, we should have an CLI executable
that should be configurable (used glyphs etc)  
SDFs of the glyphs should be packed in some way to a texture atlas. What about hinting?

A good inspirational source is TextMeshPro!

Is there any known file format for SDF Fonts?
