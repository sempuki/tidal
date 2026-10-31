// tidal-rules: ATTR-17
// tidal-expect: reject
// tidal-error-clang: attribute with scope specifier cannot follow default scope specifier
// tidal-error-gcc: attribute using prefix used together with scoped attribute token
// tidal-error-edg: attribute namespace may not be used here
// tidal-note: Gap review 2026-10-31.
[[using gnu: gnu::unused]] int x;
