// tidal-rules: STG-12 STG-13
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
struct Record { char tag; int count; double weight; };
int main() {
  Record record{'a', 3, 1.5};
  unsigned char saved[sizeof(Record)];
  unsigned char* bytes = reinterpret_cast<unsigned char*>(&record);
  for (unsigned i = 0; i != sizeof(Record); ++i) saved[i] = bytes[i];
  record = Record{'z', 9, 0.0};
  for (unsigned i = 0; i != sizeof(Record); ++i) bytes[i] = saved[i];
  return record.tag == 'a' && record.count == 3 && record.weight == 1.5 ? 0 : 1;
}
