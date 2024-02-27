use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <img src="/assets/img/couriers.png" alt="Couriers" />
        <h1>{"Index"}</h1>
    }
}
