use leptos::*;
use leptos_meta::*;
//use leptos_router::*;

mod components;

use crate::api::converse;
use crate::app::components::chat_area::ChatArea;
use crate::app::components::type_area::TypeArea;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (conversation, set_conversation) = create_signal(Conversation::new());
    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.to_string(),
            //text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });
        converse(conversation.get())
        // TODO: send message to server
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };
            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <h1>"This is the Llama chat"</h1>
        <Title text="Llama Chat"/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
    //view! {
    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    //    <Stylesheet id="leptos" href="/pkg/rusty-llma.css"/>

    // sets the document title
    //  <Title text="Welcome to my site and start chat"/>

    // content for this welcome page
    // <Router>
    //     <main>
    //         <Routes>
    //             <Route path="" view=ChatPage/>
    //             <Route path="/chatpage" view=ChatPage/>
    //             <Route path="/*any" view=NotFound/>
    //         </Routes>
    //     </main>
    // </Router>
    //}
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos dear user!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
#[component]
fn ChatPage() -> impl IntoView {
    // provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());
    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.to_string(),
            //text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });
        converse(conversation.get())
        // TODO: send message to server
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };
            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <h1>"This is the Llama chat"</h1>
        <Title text="Llama Chat"/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
