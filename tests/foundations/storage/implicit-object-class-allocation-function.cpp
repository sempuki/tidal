// tidal-rules: STG-8
// tidal-expect: run
struct Pool {
  static void* operator new(decltype(sizeof 0) size) { return ::operator new(size); }
  static void operator delete(void* p) { ::operator delete(p); }
};
struct Sample { int count; float mean; };
int main() {
  void* raw = Pool::operator new(sizeof(Sample));
  Sample* sample = static_cast<Sample*>(raw);
  sample->count = 2;
  sample->mean = 0.5f;
  bool ok = sample->count == 2 && sample->mean == 0.5f;
  Pool::operator delete(raw);
  return ok ? 0 : 1;
}
