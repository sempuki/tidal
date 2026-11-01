// tidal-rules: STG-3
// tidal-expect: run
struct Tag {};
struct Holder : Tag { Tag member; };
static_assert(sizeof(Holder) > 1);
int main() {
  Holder holder;
  Tag* base = &holder;
  Tag* member = &holder.member;
  return base != member ? 0 : 1;
}
