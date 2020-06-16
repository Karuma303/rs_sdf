# Nützliche Links

1) [Drawing Text with Signed Distance Fields in Mapbox GL](https://blog.mapbox.com/drawing-text-with-signed-distance-fields-in-mapbox-gl-b0933af6f817)  
Interessanter Artikel, wie MapBox mit SDF die Labels auf den Karten zeichnet - Inklusive Code für den FragmentShader

1) [Signed Distance Fields](https://github.com/chriscummings100/signeddistancefields/blob/master/Assets/SignedDistanceFields/SignedDistanceFieldGenerator.cs)  
C# Code Repo für einen SDF Generator mit unterschiedlichen Algorithmen. 8PSSDT sweep or a brute force eikonal algorithm

1) [Shader Fun](https://shaderfun.com/)  
Blogpost Serie über SDFs

1) [Distance Field Fonts](https://github.com/libgdx/libgdx/wiki/Distance-field-fonts)  
Gute Erklärung, wie SDF Fonts in libGDX benutzt werden  

1) [Ronjas Shader Tutorials](https://www.ronja-tutorials.com/2018/11/10/2d-sdf-basics.html)
2D Signed Distance Field Basics - Blog mit mehreren Beiträgen zum Thema SDF

## fluent syntax

```rust
    let gen = DistanceGenerator()
        .withConfiguration(GeratorConfiguration())
        .fromInput(ImageFileInput("/path"))
        .toOutput(FileOutput("/path")) // also ImageBufferOutput
        .generate();
```

#### Erstversion
```rust
let gen = DistanceGenerator::new()
            .input("/path")
            .output("/path")
            .generate();
```

## Input
Aus ImageBuffer  
Aus File (File -> ImageBuffer)  
Aus TTF Font Datei  
Zusätzlich können Transformer eingesetzt werden:
Rotate, Flip, B/W, ChannelMatrix (Kanäle anders mappen), Threshold,
scale (up/down) <- ganz wichtig ... inklusive linear/bilinear/triliniear interpolation
Bits pro Kanal (1/8/16/32/64) (Ausgangsformat) - Aber wie sieht dann das Format zum berechnen aus? B/W vs. Greyscale

**Output**  
ImageBuffer
File (wobei File nur eine bestimmte Umwandlung des ImageBuffers ist. Welche Channel werden hier genutzt? 8/16/32 Bit? Int oder Float?
Auch hier sind wieder alle möglichen Transformationen möglich (scale, rotate, flip)
welche cannel

**Configuration**  
Verwendeter Algorithmus (Naiv, 8SED, Dead Reckoning, etc.)

Das Ganze sollte auch auf TTF Fonts anwendbar sein. Hier wäre dann wichtig, dass man das Tool
im Batch aufrufen kann -> irgendein Tool basteln, was festgelegte Glyphen erzeugen kann.
entweder packt man dann die glyphen in eine max bounding box, oder (besser) jede glphe bekommt
entsprechend ihrer größe eine bestimmmte box und zusätzlich wird im output das ganze hinting
noch mit abgespeichert.

Natürlich muss auch eine Atlas Textur mit einem Packer erzeugt werden
Das wäre dann aber schon echt ein heftiges Tool, was sehr stark in die Richtung TextMeshPro geht ;)

Gibt es ein Format für SDF Fonts?
