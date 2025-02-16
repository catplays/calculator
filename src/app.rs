use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref(); // 使用 use_node_ref 创建一个引用，用于绑定到 <input> 元素，方便获取输入框的值

    let name = use_state(|| String::new()); // 创建一个状态变量，用于存储用户输入的名字

    let greet_msg = use_state(|| String::new()); // 创建一个状态变量，用于存储从 Tauri 后端返回的问候消息。
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(name2, move |_| {
            // use_effect_with 监听name2的 的变化，当 name 更新时触发
            spawn_local(async move {
                // 在异步上下文中执行代码。
                if name.is_empty() {
                    return;
                }
                // serde_wasm_bindgen：用于将 Rust 数据结构序列化为 JavaScript 可用的格式
                let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
                // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
                // 调用 Tauri 后端的 greet 命令，传递 name 作为参数。
                let new_msg = invoke("greet", args).await.as_string().unwrap();
                // 将 Tauri 返回的问候消息更新到 greet_msg 状态中
                greet_msg.set(new_msg);
            });

            || {}
        });
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        // 创建一个回调函数，用于处理表单提交事件。
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // 阻止表单的默认提交行为。
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(), // greet_input_ref 转换为 HtmlInputElement，以便获取输入框的值。
            );
        })
    };

    html! {
        <main class="container">
            <h1>{"Welcome to Tauri + Yew"}</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>
            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>
            <p>{ &*greet_msg }</p>


        </main>
    }
}
