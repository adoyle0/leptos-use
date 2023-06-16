use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_css_var_with_options, UseCssVarOptions};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let el = create_node_ref(cx);
    let (color, set_color) =
        use_css_var_with_options(cx, "--color", UseCssVarOptions::default().target(el));
    let switch_color = move |_| {
        if color() == "#df8543" {
            set_color("#7fa998".to_string());
        } else {
            set_color("#df8543".to_string());
        }
    };

    let elv = create_node_ref(cx);
    let (key, set_key) = create_signal(cx, "--color".to_string());
    let (color_val, _) = use_css_var_with_options(cx, key, UseCssVarOptions::default().target(elv));
    let change_var = move |_| {
        if key() == "--color" {
            set_key("--color-one".to_string());
        } else {
            set_key("--color".to_string());
        }
    };
    let style = move || {
        format!(
            "--color: #7fa998; --color-one: #df8543; color: {}",
            color_val()
        )
    };

    view! { cx,
        <div>
            <div node_ref=el style="--color: #7fa998; color: var(--color)">
                "Sample text, " {color}
            </div>
            <button on:click=switch_color>"Change color value"</button>
        </div>

        <div>
            <div node_ref=elv style=style class="mt-4">
                "Sample text, " {key} ": " {color_val}
            </div>
            <button on:click=change_var>"Change color variable"</button>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}