// Probe: define the ordering constants inside the class.
namespace std {
struct strong_ordering {
  signed char value;
  static constexpr strong_ordering less{-1};
};
}
int main() { return 0; }
