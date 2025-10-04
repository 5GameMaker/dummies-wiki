# Custom Coroutine Dispatcher

To be able to use `MainScope()`, you must have an implementation for `MainCoroutineDispatcher`.

Kotlin-provided dispatchers:
- `kotlinx-coroutines-javafx`
- `kotlinx-coroutines-swing`
- `kotlinx-coroutines-android`

If those are not enough, you must implement `MainCoroutineDispatcher` and `MainDispatcherFactory`.

```kt
@file:OptIn(kotlinx.coroutines.InternalCoroutinesApi::class)

package com.example.dispatcher 

internal class MainDispatcher: MainCoroutineDispatcher() {
    override fun dispatch(context: CoroutineContext, block: Runnable) {
        // FIXME: Replace this with something else.
        block.run()
    }

    // FIXME: Replace this with a different dispatcher.
    //
    //        I have no idea why cuz it works anyway but oh well.
    override val immediate = this
}

internal class DispatcherFactory: MainDispatcherFactory {
    override fun createDispatcher(allFactories: List<MainDispatcherFactory>): MainCoroutineDispatcher = MainDispatcher()

    override fun hintOnError(): String = "For tests Dispatchers.setMain from kotlinx-coroutines-test module can be used"

    override val loadPriority: Int
        get() = Int.MAX_VALUE / 2
}
```

Then, `DispatcherFactory` (or whatever you have named it) must be registered in `/META-INF/services/kotlinx.coroutines.internal.MainDispatcherFactory`

```
com.example.dispatcher.DispatcherFactory
```

## Handling exceptions

> ⚠️
> 
> Still collecting info!

- A handling service can be set via `/META-INF/services/kotlinx.coroutines.CoroutineExceptionHandler`.
- <https://github.com/Kotlin/kotlinx.coroutines/blob/f4f519b36734238ec686dfaec1e174086691781e/ui/kotlinx-coroutines-android/src/AndroidExceptionPreHandler.kt>

## References

- DuckAI, for once it was useful
- [Dispatchers class](https://kotlinlang.org/api/kotlinx.coroutines/kotlinx-coroutines-core/kotlinx.coroutines/-dispatchers/)
- [JavaFX dispatcher](https://github.com/Kotlin/kotlinx.coroutines/tree/master/ui/kotlinx-coroutines-javafx)
- [Android dispatcher](https://github.com/Kotlin/kotlinx.coroutines/tree/master/ui/kotlinx-coroutines-android)
