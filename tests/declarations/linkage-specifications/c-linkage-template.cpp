// tidal-rules: LSP-2
// tidal-expect: reject
// tidal-error-clang: templates must have C\+\+ linkage
// tidal-error-gcc: template with C linkage
// tidal-error-edg: may not have extern .C. linkage
extern "C" { template <class T> void tf(T); }
