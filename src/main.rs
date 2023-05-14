use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |change| {
            let new_value = *counter + change;
            counter.set(new_value);
        }
    };
    let increment = {
        let onclick = onclick.clone();
        move |_| {
            onclick(1);
        }
    };
    let decrement = {
        let onclick = onclick;
        move |_| {
            onclick(-1);
        }
    };

    html! {
        <div class="w-screen flex">
            <div class="bg-black text-neutral-200 flex items-center mx-auto">
                <button onclick={decrement}
                    class="bg-red-700 rounded-xl px-4 py-2 m-4">
                    { "Decrement" }
                </button>
                <p class="text-3xl font-bold">
                    { *counter }
                </p>
                <button onclick={increment}
                    class="bg-green-700 rounded-xl px-4 py-2 m-4">
                    { "Increment" }
                </button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
