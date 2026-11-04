// tidal-rules: DER-6 DER-7 DER-12 DER-13 DER-14 DER-16 ACC-4 ACC-8 ACC-9 MPTR-2 MPTR-4 CAST-15
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Write suite W9, program mixin.cpp (reports/clean-room/W9/report.md). The rules are the ones in the report's construct-to-rule notes that the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// tidal-note: Q-0353. During constant evaluation EDG 7.0 loses the member that the result of Widget::count_field names.
// W9: mixin bases with protected state.
// Rules used: DER-2, DER-6, DER-7, DER-12, DER-13, DER-14, DER-16,
// ACC-4, ACC-8, ACC-9, MPTR-2, MPTR-4, CAST-15.

// A mixin that counts events. Its state and its constructor are
// protected, so only derived classes use them (DER-14, ACC-4).
class Counter {
protected:
  int count = 0;
  constexpr Counter() = default;
  constexpr void bump() { ++count; }
public:
  constexpr int total() const { return count; }
};

// A mixin that holds a name.
class Named {
protected:
  const char* name_;
  constexpr explicit Named(const char* n) : name_(n) {}
public:
  constexpr const char* name() const { return name_; }
};

// A CRTP mixin: it reaches the derived class with static_cast (DER-16).
template <class Self>
class Doubling {
protected:
  constexpr Doubling() = default;
public:
  constexpr int doubled() const {
    return 2 * static_cast<const Self&>(*this).value();
  }
};

// Widget mixes in all three. Named is a private base, and the
// using-declaration makes Named::name public again (ACC-8, ACC-9).
class Widget : public Counter, private Named, public Doubling<Widget> {
public:
  using Named::name;
  constexpr explicit Widget(const char* n) : Named(n) {}
  constexpr void click() { bump(); }
  constexpr int value() const { return count + 10; }
  // ACC-4: protected state through another Widget is fine.
  constexpr bool same_count(const Widget& other) const {
    return count == other.count;
  }
  // ACC-4: a pointer to the protected member is formed with Widget.
  static constexpr int Widget::* count_field() { return &Widget::count; }
};

// A shared mixin through virtual bases (DER-6). One Log subobject.
struct Log {
  int lines = 0;
  explicit Log(int start) : lines(start) {}
};
struct Clickable : virtual Log {
  Clickable() : Log(100) {}         // no effect in Icon (DER-7)
  void click() { ++lines; }
};
struct Draggable : virtual Log {
  Draggable() : Log(200) {}
  void drag() { lines += 10; }
};
struct Icon : Clickable, Draggable {
  Icon() : Log(1) {}                // DER-13: the most derived class
};

constexpr int widget_check() {
  Widget w{"ok"};
  Widget v{"no"};
  w.click();
  w.click();
  v.click();
  if (w.total() != 2) return 1;
  if (w.same_count(v)) return 2;
  v.click();
  if (!w.same_count(v)) return 3;
  if (w.name()[0] != 'o') return 4;
  if (w.doubled() != 24) return 5;
  int Widget::* f = Widget::count_field();
  w.*f = 7;
  if (w.total() != 7) return 6;
  return 0;
}

static_assert(widget_check() == 0);

int main() {
  if (widget_check() != 0) return 1;
  Icon i;
  i.click();
  i.drag();
  if (i.lines != 12) return 2;      // 1 + 1 + 10
  Clickable& c = i;
  Draggable& d = i;
  Log& l1 = c;
  Log& l2 = d;
  if (&l1 != &l2) return 3;         // one shared Log
  return 0;
}
