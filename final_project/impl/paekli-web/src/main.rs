use gloo::{dialogs::alert, net::http::Request};
use leptos::*;
use leptos_use::UseWebsocketReturn;
use paekli_core::http_api::{ReceiveResponse, SendRequest};

fn main() {
    console_error_panic_hook::set_once();

    let (get_input, set_input) = create_signal(String::default());

    let send_paekli = move |_| {
        let content = get_input.get();
        let request = Request::post("https://paekli.buenzli.dev/paekli")
            .json(&SendRequest {
                content,
                recipient: None,
                express: false,
            })
            .unwrap();
        spawn_local(async {
            request.send().await.unwrap();
        });
        set_input.set(String::default());
    };

    let receive_paekli = move |_| {
        let request = Request::delete("https://paekli.buenzli.dev/paekli");
        spawn_local(async {
            let resp = request
                .send()
                .await
                .unwrap()
                .json::<ReceiveResponse>()
                .await;
            match resp {
                Ok(ReceiveResponse { content }) => {
                    alert(&format!("here's your paekli:\n{}!", content))
                }
                Err(_) => alert("no paekli"),
            }
        });
    };

    let (get_should_render, set_should_render) = create_signal(false);
    let toggle_should_render = move |_| set_should_render.update(|prev| *prev = !*prev);

    mount_to_body(move || {
        view! {
            <h1>Hello WebAssembly!</h1>
            <button on:click=send_paekli>
                Send
            </button>
            <input
                placeholder="paekli content"
                prop:value=move || get_input.get()
                on:input=move |e| set_input.set(event_target_value(&e))
            ></input>
            <button on:click=receive_paekli>
                Receive
            </button>
            <button on:click=toggle_should_render>
                toggle rendering
            </button>
            <Show when=move || get_should_render.get()>
                <NotificationListener recipient=String::from("alice") />
            </Show>
        }
    })
}

#[component]
fn NotificationListener(recipient: String) -> impl IntoView {
    let url = format!("ws://localhost:4200/notifications/{recipient}");
    let UseWebsocketReturn { message, .. } = leptos_use::use_websocket(&url);

    create_effect(move |_| {
        if let Some(message) = message.get() {
            gloo::dialogs::alert(&message);
        }
    });

    view! {
        "Listening to notifications...!"
    }
}
