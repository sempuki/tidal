// tidal-rules: CONV-17 COND-16
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion from derived class
// tidal-error-gcc: is an ambiguous base of
// tidal-error-edg: base class .Root. is ambiguous
// tidal-note: Roadmap round 9 (I4-6), I4 reduction r4-ambiguous-base.cpp. The class Both has the class Root as a base on two paths. A Both lvalue meets a Root lvalue. The conversion to Root counts for COND-16 and then cannot be made, because the base is ambiguous.
struct Root {};
struct Left : Root {};
struct Right : Root {};
struct Both : Left, Right {};
int main() {
  Both b;
  Root r;
  (void)(true ? b : r);
  return 0;
}
