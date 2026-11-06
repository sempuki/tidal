// A logger shared by two units: an inline function with a static local
// (one object for the program), an inline variable, a class template
// whose members are defined in the header, and a function declared in
// the header and defined in one unit.
#ifndef LOGGER_H
#define LOGGER_H

enum class Level { debug, info, error };

inline constexpr int max_entries = 8;

template <class Entry>
class Ring {
  Entry items_[max_entries]{};
  int next_ = 0, count_ = 0;
public:
  void push(const Entry& e) {
    items_[next_] = e;
    next_ = (next_ + 1) % max_entries;
    if (count_ < max_entries) ++count_;
  }
  int count() const { return count_; }
  const Entry& newest() const { return items_[(next_ + max_entries - 1) % max_entries]; }
};

struct Record {
  Level level = Level::debug;
  int code = 0;
};

inline Ring<Record>& log() {
  static Ring<Record> ring;
  return ring;
}

inline int errors_seen = 0;

void report(Level level, int code);  // defined in sink.cpp
int count_from_other_unit();
#endif
