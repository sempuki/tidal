// Probe: DEP-20. A return type at namespace scope names a dependent
// member type without typename. The member type is private.
// Expected from the specification: accept (access is a doubt).
template <class T> class Ring {
  struct Cell { T value; };
  Cell cell{};
public:
  Cell* first();
};
template <class T>
Ring<T>::Cell* Ring<T>::first() { return &cell; }
int main() { Ring<int> r; return r.first()->value; }
