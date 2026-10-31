// tidal-rules: SW-3
// tidal-expect: reject
// tidal-error-clang: case. statement not in switch statement
// tidal-error-gcc: not within a switch statement
// tidal-error-edg: case label may only be used within a switch
void f() {
  case 1: ;
}
