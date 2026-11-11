// tidal-rules: ADD-14 STG-14
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Read of the RV7 repair diff, RV8-R-9, and the read of the RV8 repair diff, RV9-R-3. An unsigned* points to an element of an int array. The pointer's type is the unsigned type that matches the element type, so ADD-14 counts the element as an array of one, end is the position one past it, end - p is 1, and the read through p is valid (STG-14). Both reads make every step of such a pointer undefined, and the read of the RV8 repair diff also makes the difference end - p undefined (Q-0442). Every compiler that runs the program accepts the step and the difference, and no sanitizer reports them.
int main() {
  int values[4] = {1, 2, 3, 4};
  unsigned* p = reinterpret_cast<unsigned*>(&values[0]);
  unsigned* end = p + 1;
  return (end - p == 1 && *p == 1u) ? 0 : 1;
}
