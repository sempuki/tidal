// tidal-rules: STG-13
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Q-0327. A byte pointer made from the address of the member second moves back to the first byte of the whole object and forward to one past its last byte.
struct Pair { int first; int second; };
int main() {
  Pair pair{1, 2};
  const unsigned char* whole = reinterpret_cast<const unsigned char*>(&pair);
  const unsigned char* member = reinterpret_cast<const unsigned char*>(&pair.second);
  const unsigned char* start = member - sizeof(int);
  const unsigned char* end = member + sizeof(int);
  int sum = 0;
  for (const unsigned char* p = start; p != end; ++p) sum += *p;
  bool ordered = whole < member && member < whole + sizeof pair && end == whole + sizeof pair;
  return start == whole && ordered && sum == 3 ? 0 : 1;
}
