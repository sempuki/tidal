// tidal-rules: ADD-14
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: Roadmap round 11, programmer probe typed_step_two_past_byte.cpp. first is an int* that points to the first byte of pool (STG-29). ADD-14 counts that byte as an array of one int, so first + 2 moves past the position one past it, and the behavior is undefined. No sanitizer configuration in the matrix stops the move, so the matrix only builds the program.
alignas(int) unsigned char pool[4 * sizeof(int)];
int main() {
  int* first = static_cast<int*>(static_cast<void*>(pool));
  int* far = first + 2;                  // undefined: past one past the byte
  return far == first ? 1 : 0;
}
