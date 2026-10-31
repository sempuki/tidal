// tidal-rules: NS-12
// tidal-expect: reject
// tidal-error-clang: non-inline namespace cannot be reopened as inline
// tidal-error-gcc: inline namespace must be specified at initial definition
// tidal-error-edg: was not declared inline
namespace A {
  namespace V {}
  inline namespace V {}
}
