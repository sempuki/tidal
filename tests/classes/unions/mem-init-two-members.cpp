// tidal-rules: UNI-8
// tidal-expect: reject
union U { int i; float f; U() : i(1), f(2.0f) {} };
