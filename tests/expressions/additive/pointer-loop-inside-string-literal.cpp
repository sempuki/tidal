// tidal-rules: ADD-6 ADD-10
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction literal_pointer_range_loop.cpp. A loop steps a pointer from the first character of a string literal to the position five characters on, which is inside the array of six characters, so no step leaves the array.
int main() {
  const char* p = "tidal";
  const char* stop = p + 5;
  int vowels = 0;
  for (; p != stop; ++p)
    vowels += *p == 'i' || *p == 'a';
  return vowels == 2 ? 0 : 1;
}
