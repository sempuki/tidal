// tidal-rules: ODR-13 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: multiple definition of .Tag<char>::count
// tidal-error-gcc: multiple definition of .Tag<char>::count
// tidal-sources: explicit-class-member-in-two-units.tu.cpp
#include "tag-template.h"
int main() { return &Tag<char>::count == tag_elsewhere() ? 0 : 1; }
