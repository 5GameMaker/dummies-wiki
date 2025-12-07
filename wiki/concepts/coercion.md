# Type Coercion

For strictly typed languages, it can be assumed that for unrelated types `A` and `B`,
`B` cannot be used in place of `A` nor `A` can be used in place of `B`.

## Subtypes and coercion

In languages like Java, type `A` can be made a *subtype* of `B` with a syntax like
```java
class B {}

class A extends B {}
//    ^^^^^^^^^^^ declaring A to be a subtype of B
```

Thus, `A` can be used in place of `B` (for the most part):
```java
// Both are allowed since A is a subtype of B
B a = new A();
B b = new B();
```

This is called *coercion*, `A` can be *coerced* to `B`, thus
using an `A` in place of `B` is valid:

```java
void doThing(B value);

doThing(new A());
```

## Variance

Variance describes coercion of a container type when its
type parameter is specified:

For type `C<T>`, `A` extending `B` (multiple may apply):
- Covariant if `C<A>` coerces to `C<B>`
- Contravariant if `C<B>` coerces to `C<A>`
- Invariant if `C<A>` does not coerce to `C<B>` and `C<B>` does not coerce to `C<A>`

In Java, for a container type `C<T>` and `A` extending `B`,  
`C<A>` cannot be coerced to `C<B>`, nor vice versa.

This is known as *invariance*, all generic types in Java are
invariant.

Kotlin, however, has variance specifiers[^1]:
- `out` for covariance
- `in` for countervariance

Thus:
- Covaraiance
```kotlin
class C<out T>

open class B
class A: B()

val a: C<B> = C<A>()
val b: C<B> = C<B>()
```
- Contravariance
```kotlin
class C<in T>

open class B
class A: B()

val a: C<A> = C<A>()
val b: C<A> = C<B>()
```

Similar concepts can be found in other languages[^2][^3]

[^1]: https://kotlinlang.org/docs/generics.html
[^2]: https://blog.loitzl.com/posts/variance-and-covariance-explained-with-csharp/
[^3]: https://doc.rust-lang.org/reference/subtyping.html#r-subtyping.variance
