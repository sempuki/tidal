// tidal-rules: CPT-11
// tidal-expect: accept
// tidal-note: A generic lambda's call operator is a function template, also outside any template.
auto keep = [](auto x) requires (sizeof(x) > 1) { return x; };
int kept = keep(1);
