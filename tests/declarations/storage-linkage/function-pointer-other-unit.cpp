// tidal-rules: LNK-2
// tidal-expect: run
// tidal-sources: function-pointer-other-unit.tu.cpp
// tidal-note: Roadmap round 7, programmer reduction fnptr_other_unit/main.cpp and other.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). A pointer to a function with internal linkage in another unit can be called.
using Fn = int (*)(int);
Fn pick();
int main() { return pick()(3) == 6 ? 0 : 1; }
