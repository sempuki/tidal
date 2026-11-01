// tidal-rules: STG-2 STG-13
// tidal-expect: run
struct Pair { int first; long second; };
int main() {
  Pair pair{1, 2};
  const unsigned char* start = reinterpret_cast<const unsigned char*>(&pair);
  const unsigned char* end = start + sizeof pair;
  const unsigned char* first = reinterpret_cast<const unsigned char*>(&pair.first);
  const unsigned char* second = reinterpret_cast<const unsigned char*>(&pair.second);
  bool at_start = first == start;
  bool inside = second > start && second + sizeof pair.second <= end;
  return at_start && inside ? 0 : 1;
}
