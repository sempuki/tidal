// tidal-rules: STG-16
// tidal-expect: run
struct Head { int id; };
struct Node { Head head; int next; };
union Cell { Node node; long raw; };
int main() {
  Cell cell{.node = {{5}, 6}};
  int* id = reinterpret_cast<int*>(&cell);   // union, member, first member, first member
  Head* head = static_cast<Head*>(static_cast<void*>(id));
  Cell* back = reinterpret_cast<Cell*>(id);
  return *id == 5 && head->id == 5 && back == &cell ? 0 : 1;
}
