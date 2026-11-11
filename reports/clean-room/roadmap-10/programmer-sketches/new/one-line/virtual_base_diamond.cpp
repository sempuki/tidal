// A diamond with a virtual base: the most derived class constructs the base once.
struct Node { int id; Node(int i) : id(i) {} virtual ~Node() = default; };
struct Left : virtual Node { Left() : Node(1) {} };
struct Right : virtual Node { Right() : Node(2) {} };
struct Join : Left, Right { Join() : Node(3) {} };
int main() {
  Join j;
  Left& l = j;
  Right& r = j;
  return (&static_cast<Node&>(l) == &static_cast<Node&>(r) && j.id == 3) ? 0 : 1;
}
