# bezier-kit
Inside of [ASAM OpenDRIVE](https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/latest/specification/index.html) we can find Bezier-Curves everywhere. They are kind of hidden.
A [LaneWidth](https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/latest/specification/11_lanes/11_06_lane_geometry.html#sec-8d8ac2e0-b3d6-4048-a9ed-d5191af5c74b) with its `a, b, c, d` Parameters is essetialy a 1D-Bezier-Curve represented as [Bernstein_polynomial](https://en.wikipedia.org/wiki/Bernstein_polynomial).

Generic **1D**, **2D** and **3D** bezier curves with a focus on efficient **flattening**.



# Acknowledges
- The flattening is a reimplementation of the following post [flattening-bezier-curves-and-arcs](https://minus-ze.ro/posts/flattening-bezier-curves-and-arcs/)

# Similar Libaries
- [kurbo](https://docs.rs/kurbo/0.11.1/kurbo/index.html) 
    - good flattening but 
    - godd struct definitions
    - only `2D` `f64` support.
- [bezier-nd-rs](https://docs.rs/bezier-nd/latest/bezier_nd/) good generic Point type but no efficient `flattening`
- [bezier_rs](https://docs.rs/bezier-rs/latest/bezier_rs/)
    - no flattening and only `2D`
- [vek](https://docs.rs/vek/0.17.1/vek/)

- [lyon](https://docs.rs/lyon_geom/1.0.6/lyon_geom/index.html) \
    - good `flattening` 
    - only `2D` `f64` support.

- [flo_curves](https://docs.rs/flo_curves/latest/flo_curves/index.html) generic types but no `flattening` but a good `curve-fitting`
    - maybe its better to extend `flo_curves` ?
    - mainly only `cubic` bezier

- [FlatteningQuadraticBezierCurves](https://github.com/AlexandruIca/FlatteningQuadraticBezierCurves)
    - example for flattening quadtratic bezier curves



## Naming
- eval a Curve
    -  eval (kurbo)
    -  point_at_pos(flo_curves)
    -  point_at(bezier-nd-rs)
    -  evaluate(bezier-rs)
    -  sample(lyon)