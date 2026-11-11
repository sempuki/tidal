// tidal-rules: UNI-14 STG-6
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction construct_in_member_of_inactive_union_member.cpp, at run time. Q-0425. Gap review RV6-A-14. The union Outer has no active member, and placement new builds a double at the address of the member d of the member inner. At run time the construction is valid (UNI-14), and the program reads the new double through the pointer that new returns. GCC and Clang run the program, also with the sanitizers.
#include "../../foundations/storage/placement.h"
union Inner { int i; double d; Inner() {} };
union Outer { char tag; Inner inner; Outer() {} };
int main() {
  Outer o;
  double* d = new (&o.inner.d) double(1.5);
  return *d == 1.5 ? 0 : 1;
}
