// tidal-rules: STG-3 CLS-35
// tidal-expect: run
struct Tag {};
struct Value : Tag { int number; };
static_assert(sizeof(Value) == sizeof(int));
int main() {
  Value value{{}, 4};
  Tag* base = &value;
  return static_cast<void*>(base) == static_cast<void*>(&value.number) ? 0 : 1;
}
