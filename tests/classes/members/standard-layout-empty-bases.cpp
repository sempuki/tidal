// tidal-rules: CLS-19 CLS-30 CLS-33
// tidal-expect: run
// tidal-note: Empty bases at any depth leave the class standard-layout. Each base of an empty standard-layout class has size zero and shares the address of the object and of its first member.
template <class A, class B> bool same_address(const A& a, const B& b) { return static_cast<const void*>(&a) == static_cast<const void*>(&b); }
struct Tag {};
struct Mark {};
struct Deep : Tag {};
struct Record : Deep, Mark { int id; };
static_assert(sizeof(Record) == sizeof(int));
int main() {
  Record record{};
  return same_address(record, record.id) && same_address(record, static_cast<const Tag&>(record)) && same_address(record, static_cast<const Mark&>(record)) ? 0 : 1;
}
