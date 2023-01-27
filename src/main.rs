use gloo::events::EventListener;
use wasm_bindgen::JsValue;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[rustfmt::skip]
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]                  Home,
    #[at("/page1")]             Page1,
    #[at("/page2")]             Page2,
    #[not_found] #[at("/404")]  NotFound,
}

#[rustfmt::skip]
fn switch(routes: Route) -> Html {
    use Route::*;
    match routes {
        Home     => html! { <h1>{ "Home" }</h1> },
        Page1    => html! { <h1>{ "Page 1" }</h1> },
        Page2    => html! { <h1>{ "Page 2" }</h1> },
        NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn Changer() -> Html {

    // Based on https://yew.rs/docs/concepts/html/events#using-gloo-concise
    let div_node_ref = use_node_ref();
    let navigator = use_navigator().expect_throw("ChangeRoute failed to get navigator");

    use_effect_with_deps(
        {
            let div_node_ref = div_node_ref.clone();
            move |_| {
                let mut change_listener = None;
                if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                    let onchangeroute = Callback::from(move |e: Event| {

                        // ty Jonas Bojesen https://stackoverflow.com/questions/63604399
                        let Ok(jspath) = js_sys::Reflect::get(&e, &JsValue::from_str("detail")) else {return;};
                        let Some(path) = jspath.as_string() else {return;};

                        // https://docs.rs/yew-router/latest/yew_router/trait.Routable.html#tymethod.recognize
                        let Some(route) = Route::recognize(&path) else {return;};
                        navigator.push(&route);
                    });

                    let listener = EventListener::new(&element, "change-route", move |e| {
                        onchangeroute.emit(e.clone())
                    });

                    change_listener = Some(listener);
                }

                move || drop(change_listener)
            }
        },
        div_node_ref.clone(),
    );

    html! {
        <div ref={div_node_ref} id="change-route"></div>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Changer/>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    let yew_main = gloo::utils::document()
        .get_element_by_id("yew-main")
        .unwrap();
    yew::Renderer::<Main>::with_root(yew_main).render();
}
