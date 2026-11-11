// tidal-rules: ACC-5 TYN-4
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer reduction friend_enclosing_class_in_nested_class.cpp. Lookup of Queue in the nested Cursor finds the enclosing class, so friend class Queue names that class and declares no new one (TYN-4). The friend declaration gives Queue access to Cursor and changes nothing for Cursor, which reads the private member of the complete Queue in a member function body.
class Queue {
  int items[3] = {4, 5, 6};
public:
  struct Cursor {
    friend class Queue;
    const Queue* owner;
    int at(int k) const { return owner->items[k]; }
  };
};
int main() {
  Queue q;
  Queue::Cursor c{&q};
  return c.at(2) == 6 ? 0 : 1;
}
