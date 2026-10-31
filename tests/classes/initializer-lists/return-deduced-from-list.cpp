// tidal-rules: ILC-10
// tidal-expect: reject
// tidal-error-clang: cannot deduce return type from initializer list
// tidal-error-gcc: returning initializer list
// tidal-error-edg: brace-enclosed list does not provide a return type
auto make() { return {1, 2}; }
