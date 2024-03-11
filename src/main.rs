use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
            {count.get()}
        </button>
        <progress
    max="50"
    value=move || count.get()78df364  asdfasdfasdfasdasdfasdfasdfasdffdfds * 2
/>
        <progress
    max="50"
    value=double_count
/>
<p>
    "Double Count: "
    {double_count}
</p>
    }
}



fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
}

