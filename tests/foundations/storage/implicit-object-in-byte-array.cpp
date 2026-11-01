// tidal-rules: STG-7 STG-8
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
int main() {
  alignas(int) unsigned char buffer[sizeof(int)];
  int* number = reinterpret_cast<int*>(buffer);
  *number = 5;
  return *number == 5 ? 0 : 1;
}
