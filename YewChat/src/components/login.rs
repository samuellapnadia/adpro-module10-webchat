use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div
            class="w-screen h-screen bg-pink-100 flex items-center justify-center"
        >
            <div class="bg-white bg-opacity-90 p-8 rounded-2xl shadow-lg text-center max-w-sm w-full">
                <h1 class="text-pink-600 text-2xl font-extrabold mb-2">{"Welcome to YewChat 💌"}</h1>
                <p class="text-pink-400 text-sm mb-6">{"Type your username to start chatting!"}</p>
                <form class="space-y-4">
                    <input
                        {oninput}
                        class="w-full p-3 rounded-full border-2 border-pink-300 placeholder-pink-300 focus:outline-none focus:ring-2 focus:ring-pink-400"
                        placeholder="Your username..."
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="w-full bg-pink-400 hover:bg-pink-500 text-white font-semibold py-2 rounded-full shadow-md transition disabled:bg-pink-200"
                        >
                            {"💬 Start Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
    
    
}