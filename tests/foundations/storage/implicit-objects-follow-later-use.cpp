// tidal-rules: STG-24 STG-8
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
int main(int argc, char**) {
  void* raw = ::operator new(sizeof(long) > 2 * sizeof(int) ? sizeof(long) : 2 * sizeof(int));
  int result;
  if (argc > 5) {
    long* whole = static_cast<long*>(raw);   // one long
    *whole = 7;
    result = *whole == 7 ? 0 : 1;
  } else {
    int* halves = static_cast<int*>(raw);    // an array of int
    halves[0] = 3;
    halves[1] = 4;
    result = halves[0] + halves[1] == 7 ? 0 : 1;
  }
  ::operator delete(raw);
  return result;
}
