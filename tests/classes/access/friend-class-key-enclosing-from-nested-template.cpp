// tidal-rules: ACC-5 TYN-4
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer reduction friend_enclosing_class_in_nested_class_of_template.cpp (from ring_first_try.cpp). In a class nested in a class template, friend class Queue finds the injected class name and names the enclosing specialization (TYN-4). Cursor then reads the private member of Queue<int> through a pointer.
template <class T> class Queue {
  T items[3] = {4, 5, 6};
public:
  struct Cursor {
    friend class Queue;
    const Queue* owner;
    T at(int k) const { return owner->items[k]; }
  };
};
int main() {
  Queue<int> q;
  Queue<int>::Cursor c{&q};
  return c.at(1) == 5 ? 0 : 1;
}
