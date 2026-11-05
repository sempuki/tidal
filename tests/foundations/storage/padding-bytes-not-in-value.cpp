// tidal-rules: STG-11 STG-12 STG-13
// tidal-expect: run
struct Gap { char tag; int count; };
int main() {
  Gap a{'a', 5};
  Gap b{'a', 5};
  unsigned char* bytes = reinterpret_cast<unsigned char*>(&b);
  const unsigned char* tag = reinterpret_cast<const unsigned char*>(&b.tag);
  const unsigned char* count = reinterpret_cast<const unsigned char*>(&b.count);
  for (unsigned char* p = bytes; p != bytes + sizeof b; ++p)
    if (p != tag && (p < count || p >= count + sizeof b.count)) *p = 0xAB;   // a padding byte
  return sizeof(Gap) > sizeof(char) + sizeof(int) && b.tag == a.tag && b.count == a.count ? 0 : 1;
}
