use std::str::FromStr;

pub struct HotConstant<T: Clone + FromStr> {
    initial_t: T,
    #[allow(unused)]
    css_var_name: &'static str,
}

impl<T: Clone + FromStr> HotConstant<T> {
    pub const fn new(initial_t: T, css_var_name: &'static str) -> Self {
        Self {
            initial_t,
            css_var_name,
        }
    }

    pub fn get(&self) -> T {
        #[cfg(feature = "enable_hot_constant")]
        return get_value_web(self.css_var_name).unwrap_or_else(|| self.initial_t.clone());

        #[cfg(not(feature = "enable_hot_constant"))]
        return self.initial_t.clone();
    }
}
#[cfg(feature = "enable_hot_constant")]
fn get_value_web<T: FromStr>(css_var_name: &'static str) -> Option<T> {
    let window: web_sys::Window = web_sys::window()?;
    let document = window.document()?;
    let cs = window
        .get_computed_style(&document.document_element()?)
        .ok()??;

    let property_value_string = cs.get_property_value(css_var_name).ok()?;

    T::from_str(&property_value_string).ok()
}
