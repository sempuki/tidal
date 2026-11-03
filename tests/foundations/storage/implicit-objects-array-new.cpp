// tidal-rules: STG-24
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: The storage comes from the array allocation function. Later code uses it as one class object or as an array of int, and either set of implicit objects gives defined behavior.
struct Pair { int first; int second; };
int main(int argc, char**) {
  void* raw = ::operator new[](sizeof(Pair));
  int result;
  if (argc > 5) {
    Pair* pair = static_cast<Pair*>(raw);
    pair->first = 3;
    pair->second = 4;
    result = pair->first + pair->second == 7 ? 0 : 1;
  } else {
    int* values = static_cast<int*>(raw);
    values[0] = 5;
    values[1] = 2;
    result = values[0] + values[1] == 7 ? 0 : 1;
  }
  ::operator delete[](raw);
  return result;
}
