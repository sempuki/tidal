// tidal-rules: SEQ-9
// tidal-expect: run
int calls = 0;
struct Item {
  int v;
  Item(int x) : v(x) { ++calls; }              // converting constructor
  Item(const Item& o) : v(o.v) { ++calls; }    // copy constructor
  ~Item() { ++calls; }                         // destructor
  operator int() const { ++calls; return v; }  // conversion function
  bool operator==(int x) const { ++calls; return v == x; }
};
struct Range {
  int data[2] = {1, 2};
  const int* begin() const { ++calls; return data; }
  const int* end() const { ++calls; return data + 2; }
};
alignas(16) unsigned char pool[64];
void* operator new(decltype(sizeof 0)) { ++calls; return pool; }
void operator delete(void*) noexcept { ++calls; }
void operator delete(void*, decltype(sizeof 0)) noexcept { ++calls; }
int take(Item i) { return i.v; }
int main() {
  int before;
  before = calls; take(5);                      // Item(int), ~Item
  if (calls - before != 2) return 1;
  Item a(1);
  before = calls; int x = a;                    // operator int
  if (calls - before != 1 || x != 1) return 2;
  before = calls; bool same = (1 == a);         // reversed operator==
  if (calls - before != 1 || !same) return 3;
  before = calls; take(a);                      // copy, ~Item
  if (calls - before != 2) return 4;
  before = calls; int* p = new int(3); delete p; // operator new, operator delete
  if (calls - before != 2 && calls - before != 0) return 5;  // NEW-14: the pair can be omitted
  before = calls; int sum = 0; for (int e : Range{}) sum += e;  // begin, end
  if (calls - before != 2 || sum != 3) return 6;
  return 0;
}
