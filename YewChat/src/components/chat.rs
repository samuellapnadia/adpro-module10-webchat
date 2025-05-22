use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
    
        html! {
            <div class="flex w-screen font-sans text-gray-800">
                <div class="flex-none w-60 h-screen bg-pink-50 shadow-inner overflow-auto">
                    <div class="text-xl font-bold p-4 text-pink-600">{"💖 Online Users"}</div>
                    {
                        self.users.iter().map(|u| {
                            html! {
                                <div class="flex items-center m-2 bg-white rounded-lg px-3 py-2 hover:bg-pink-100 transition">
                                    <img class="w-10 h-10 rounded-full border border-pink-300" src={u.avatar.clone()} alt="avatar" />
                                    <div class="ml-3">
                                        <div class="font-medium text-sm">{ &u.name }</div>
                                        <div class="text-xs text-pink-400">{"🟢 Online"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
    
                <div class="grow h-screen flex flex-col bg-white">
                    <div class="h-16 bg-white border-b shadow flex items-center px-4 text-xl font-bold text-pink-600">
                        {"💬 Hi! Welcome to YewChat!"}
                    </div>
    
                    <div class="flex-grow overflow-y-auto px-6 py-4 space-y-3 bg-pink-50">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html! {
                                    <div class="flex items-start gap-3">
                                        <img class="w-9 h-9 rounded-full border border-pink-300" src={user.avatar.clone()} />
                                        <div class="bg-white p-3 rounded-xl shadow max-w-xl">
                                            <div class="text-sm font-semibold text-pink-600">{ &m.from }</div>
                                            <div class="text-sm mt-1 text-gray-700">
                                                {
                                                    if m.message.ends_with(".gif") {
                                                        html! { <img src={m.message.clone()} class="rounded mt-2 max-w-xs" /> }
                                                    } else {
                                                        html! { <span>{ &m.message }</span> }
                                                    }
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
    
                    <div class="h-20 bg-white border-t flex items-center px-4 gap-3 shadow-inner">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Type a message 💌"
                            class="flex-grow py-2 px-4 bg-pink-100 rounded-full outline-none focus:ring-2 focus:ring-pink-400"
                            required=true
                        />
                        <button onclick={submit} class="bg-pink-500 hover:bg-pink-600 text-white rounded-full w-12 h-12 flex items-center justify-center shadow-md transition">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 fill-current">
                                <path d="M0 0h24v24H0z" fill="none"/>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
    
    
}