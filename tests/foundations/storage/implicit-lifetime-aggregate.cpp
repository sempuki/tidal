// tidal-rules: STG-7
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
struct Name { char first; Name() : first('x') {} ~Name() {} };
struct Entry { int key; Name name; };   // an aggregate; its destructor is not user-provided
int main() {
  void* raw = ::operator new(sizeof(Entry));
  Entry* entry = static_cast<Entry*>(raw);
  entry->key = 4;   // the Entry and its key exist; the Name member does not
  bool ok = entry->key == 4;
  ::operator delete(raw);
  return ok ? 0 : 1;
}
