// tidal-rules: STG-7 STG-8 STG-28
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
int main() {
  void* raw = ::operator new(sizeof(int) * 4);
  int* items = static_cast<int*>(raw);   // an int[4] exists, so items + i is valid
  for (int i = 0; i != 4; ++i) items[i] = i * 10;
  int sum = 0;
  for (int* p = items; p != items + 4; ++p) sum += *p;
  ::operator delete(raw);
  return sum == 60 ? 0 : 1;
}
