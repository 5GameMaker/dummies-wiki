# Classes

## Kinds of classes

There are 3 kinds of classes:
- Primitive types.

  They are stored on stack and cannot be `null`.

  There is no Java method to obtain a primitive class by name.

  To check if a class is primitive, you can use `Class.isPrimitive()`[^1].
- Boxed types.

  Classes. All primitives have their own corresponding boxed class (i.e. `Boolean` for `boolean`).

  Classes cannot be generic over primitives, which are stored internally as the highest superclass.

  To obtain a boxed type, you can use `Class.forName(String)`[^1].
- Array types.

  Array types don't have their own class definitions and they are allowed to be generic over primitives.

  Arrays are used internally for varargs.

  To obtain an array type, you can use `java.reflect.Array.newInstance(Class, int...)`[^2] or `Class.forName("[<descriptor>")`.

## Names

Usually this isn't explained well, so...

For a class,
```
class A {
    static class B {}
}
```

`Class<A.B>` provides 4 methods for obtaining class name:
- `getName()` for an actual JVM class name that can be used in `Class.forName`. (`A$B`)
- `getCanonicalName()` for a nicely formatted class name that you'd typically use to refer to the class in Java. (`A.B`)
- `getSimpleName()` for just a class name. (`B`)
- `descriptorString()` for a *nominal descriptor*. (`LA$B;`)

[^1]: https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html
[^2]: https://docs.oracle.com/javase/8/docs/api/java/lang/reflect/Array.html
