Euclidean distance, Manhattan distance, Chamfer Distance
https://graphics.stanford.edu/courses/cs468-03-fall/Papers/completeDistanceFieldRep.pdf
"Traditionally, distance fields are defined as spatial fields of
 scalar distances to a surface geometry or shape. Each element in a
 distance field specifies its minimum distance to the shape. As long
 as the shape is represented by an oriented manifold, positive and
 negative distances can be used to distinguish outside and inside of
 the shape, for instance, using negative values on the outside and
 positive on the inside. Distance fields have a number of applications in constructive solid geometry [1][7], surface reconstruction
 and normal estimation [9] and morphing"
 https://prideout.net/blog/distance_fields/
 The EDT (Euclidean Distance Transform) can be defined as consuming a field of booleans and producing a field of scalars such that each value in the output is the distance to the nearest “true” cell in the input.
 Another useful concept is the signed distance field (SDF) which is the subtraction of the inverted EDT from the original EDT. This is depicted in Figure 1c. Note that negative values are inside the contour of the shape and positive values are outside. SDF’s play an important role in physics simulations and certain rendering techniques.

