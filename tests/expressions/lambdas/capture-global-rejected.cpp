// tidal-rules: LAM-6
// tidal-expect: reject
int global = 2;
auto g = [global] { return global; };
