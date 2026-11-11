// A fixed-capacity buffer that owns its elements in raw storage, and an
// element type that counts how many of its objects are alive. Exits 0
// when every check holds.
import Foundation

var alive = 0

final class Token {
    let id: Int
    init(_ id: Int) { self.id = id; alive += 1 }
    deinit { alive -= 1 }
}

final class Buffer<Element> {
    private let storage: UnsafeMutablePointer<Element>
    let capacity: Int
    private(set) var count = 0

    init(capacity: Int) {
        self.capacity = capacity
        storage = UnsafeMutablePointer<Element>.allocate(capacity: capacity)
    }

    deinit {
        storage.deinitialize(count: count)
        storage.deallocate()
    }

    @discardableResult
    func append(_ value: Element) -> Bool {
        guard count < capacity else { return false }
        (storage + count).initialize(to: value)
        count += 1
        return true
    }

    func removeLast() -> Element? {
        guard count > 0 else { return nil }
        count -= 1
        return (storage + count).move()
    }

    subscript(index: Int) -> Element {
        get { storage[index] }
        set { storage[index] = newValue }
    }

    func removeAll(where shouldRemove: (Element) -> Bool) {
        var kept = 0
        for index in 0..<count {
            let value = (storage + index).move()
            if shouldRemove(value) { continue }
            (storage + kept).initialize(to: value)
            kept += 1
        }
        count = kept
    }
}

var failures = 0
func check(_ ok: Bool) { if !ok { failures += 1 } }

do {
    let buffer = Buffer<Token>(capacity: 4)
    for id in 1...5 { buffer.append(Token(id)) }
    check(buffer.count == 4)
    check(alive == 4)

    buffer[1] = Token(20)
    check(alive == 4)
    check(buffer[1].id == 20)

    if let last = buffer.removeLast() { check(last.id == 4) }
    check(alive == 3)

    buffer.removeAll { $0.id % 2 == 1 }
    check(buffer.count == 1)
    check(buffer[0].id == 20)
    check(alive == 1)
}
check(alive == 0)

do {
    let numbers = Buffer<Int>(capacity: 3)
    check(numbers.append(7) && numbers.append(8) && numbers.append(9))
    check(!numbers.append(10))
    check(numbers[0] + numbers[1] + numbers[2] == 24)
}

exit(failures == 0 ? 0 : 1)
