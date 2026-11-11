// tidal-rules: UNI-6 UNI-14
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: Roadmap round 10, programmer reduction construct_in_member_of_inactive_union_member.cpp, at run time. Q-0425. The union Outer has no active member, and placement new builds a double at the address of the member d of the member inner. The new double is not of the type of inner, so UNI-14 does not make inner active, and the construction uses a member that does not exist (UNI-6). GCC and Clang run the program with no sanitizer report, so the matrix compilers only build it. The interpreter must stop with UNI-6 (decision 0053).
#include "../../foundations/storage/placement.h"
union Inner { int i; double d; Inner() {} };
union Outer { char tag; Inner inner; Outer() {} };
int main() {
  Outer o;
  double* d = new (&o.inner.d) double(1.5);
  return *d == 1.5 ? 0 : 1;
}
