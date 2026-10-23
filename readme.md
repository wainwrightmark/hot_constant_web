# Hot Constant Web

A crate that lets you dynamically change constants in rust websites at runtime.

```rust
pub static LOZENGE_WIDTH: HotConstant<f32> = HotConstant::new(40.0, "--lozenge-width");
````


Change this value at runtime by injecting into the css. 

```css
:root { --lozenge-height:100;}
```

You can inject into css with a browser extension like https://chromewebstore.google.com/detail/css-inject/faibaacflblbomlgemniiegenfkjhopj

This will only work if you enable the feature `enable_hot_constant`. Otherwise the value will behave like a constant.