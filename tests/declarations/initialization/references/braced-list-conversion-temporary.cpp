// tidal-rules: REF-6 REF-14
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-error-gcc: cannot bind rvalue reference of type .int&&. to lvalue of type .int.
// tidal-note: Q-0157. Source is not int, so REF-14 makes a temporary int from {source}, and both references bind the temporary. GCC binds both references through operator int& and rejects the rvalue reference.
int target = 1;
struct Source { operator int&() { return target; } };
Source source;

int main() {
  const int& seen{source};
  int&& taken{source};
  target = 7;
  if (&seen == &target || &taken == &target) return 1;
  if (seen != 1 || taken != 1) return 2;
  return 0;
}
