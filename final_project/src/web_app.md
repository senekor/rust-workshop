# Web App

Interested in building a web app with Rust, I see?
I understand you.
For too long, JavaScript has held the industry hostage with its iron grip on the browser.

Even if you are not yet traumatized by JavaScript, web apps are just so darn useful.
They are extremely easy to deploy and arguably the most platform-independent way to make a GUI.

There is good news for the mental health of web developers around the globe:
[WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly) (short: wasm) is a byte code supported by all major browsers since 2017.
Finally, we have a real alternative to JavaScript!
Wasm quite low-level and intended as a compilation target.
Rust has industry-leading support for compiling to wasm, so it is a surprisingly good choice for building web apps.

```admonish note title="Requirements"
For building a web app, we need some basic knowledge about HTML.
This guide will not provide that.
If you know nothing about HTML, you should still be able to follow along until the MVP, but going beyond that will be frustrating if not impossible.
Knowledge of some JavaScript UI-framework like React, Angular, Vue etc. are benefitial but not required.
(The library we'll be using is most similar to Solid.)
If you know about CSS or Tailwind, feel free to make the app as pretty as you like! ✨
```

Due to its nature, this component is relatively library-heavy.
The browser is not really the _native_ environment of Rust, so we need a bit of cushioning to get comfortable.
We're gonna need a UI-rendering library that's made for the web, one that provides wrappers of browser APIs that are only available to JavaScript as well as an additional build tool.
