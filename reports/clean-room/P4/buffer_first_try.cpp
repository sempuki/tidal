// A port of Buffer.swift: a fixed-capacity buffer that owns its elements
// in raw storage, and an element type that counts how many of its
// objects are alive. Exits 0 when every check holds.

inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }

int alive = 0;

struct Token {
  int id;
  explicit Token(int i) : id(i) { ++alive; }
  Token(const Token& other) : id(other.id) { ++alive; }
  Token& operator=(const Token& other) {
    id = other.id;
    return *this;
  }
  ~Token() { --alive; }
};

// A value that can be absent, as Swift's Optional. It builds its value
// in its own bytes and keeps the pointer that placement new returns.
template <class T>
class Maybe {
  alignas(T) unsigned char storage[sizeof(T)];
  T* held = nullptr;
public:
  Maybe() = default;
  explicit Maybe(T&& value) : held(new (storage) T(static_cast<T&&>(value))) {}
  Maybe(Maybe&& other)
      : held(other.held ? new (storage) T(static_cast<T&&>(*other.held)) : nullptr) {}
  Maybe(const Maybe&) = delete;
  Maybe& operator=(const Maybe&) = delete;
  ~Maybe() {
    if (held) held->~T();
  }
  explicit operator bool() const { return held != nullptr; }
  T& operator*() const { return *held; }
  T* operator->() const { return held; }
};

template <class Element>
class Buffer {
  Element* storage;  // element 0 of an array of Element (STG-28)
  int count_ = 0;
  int capacity_;
public:
  explicit Buffer(int capacity)
      : storage(static_cast<Element*>(::operator new(sizeof(Element) * capacity))),
        capacity_(capacity) {}
  Buffer(const Buffer&) = delete;  // one owner for the storage
  Buffer& operator=(const Buffer&) = delete;
  ~Buffer() {
    for (int i = 0; i != count_; ++i) storage[i].~Element();
    ::operator delete(storage);
  }

  int count() const { return count_; }
  int capacity() const { return capacity_; }

  bool append(const Element& value) {
    if (count_ >= capacity_) return false;
    new (storage + count_) Element(value);
    ++count_;
    return true;
  }

  Maybe<Element> removeLast() {
    if (count_ == 0) return Maybe<Element>();
    --count_;
    Maybe<Element> last(static_cast<Element&&>(storage[count_]));
    storage[count_].~Element();
    return last;
  }

  Element& operator[](int index) { return storage[index]; }

  template <class Predicate>
  void removeAll(Predicate shouldRemove) {
    int kept = 0;
    for (int index = 0; index != count_; ++index) {
      Element value(static_cast<Element&&>(storage[index]));
      storage[index].~Element();
      if (shouldRemove(value)) continue;
      new (storage + kept) Element(static_cast<Element&&>(value));
      ++kept;
    }
    count_ = kept;
  }
};

int failures = 0;
void check(bool ok) {
  if (!ok) ++failures;
}

int main() {
  {
    Buffer<Token> buffer(4);
    for (int id = 1; id <= 5; ++id) buffer.append(Token(id));
    check(buffer.count() == 4);
    check(alive == 4);

    buffer[1] = Token(20);
    check(alive == 4);
    check(buffer[1].id == 20);

    if (auto last = buffer.removeLast()) check(last->id == 4);
    check(alive == 3);

    buffer.removeAll([](const Token& t) { return t.id % 2 == 1; });
    check(buffer.count() == 1);
    check(buffer[0].id == 20);
    check(alive == 1);
  }
  check(alive == 0);

  {
    Buffer<int> numbers(3);
    check(numbers.append(7) && numbers.append(8) && numbers.append(9));
    check(!numbers.append(10));
    check(numbers[0] + numbers[1] + numbers[2] == 24);
  }

  return failures == 0 ? 0 : 1;
}
