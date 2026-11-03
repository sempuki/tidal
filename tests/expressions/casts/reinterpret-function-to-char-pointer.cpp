// tidal-rules: CAST-12
// tidal-expect: run
// tidal-note: On the matrix targets a pointer to a function converts to char* and back, and the result calls the function.
int target(int v) { return v + 1; }
int main() {
  char* bytes = reinterpret_cast<char*>(&target);
  int (*back)(int) = reinterpret_cast<int (*)(int)>(bytes);
  return back == &target && back(3) == 4 ? 0 : 1;
}
