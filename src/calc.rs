use std::string;

use js_sys::JSON::parse;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct CalculatorState {
    display: String,                  // 展示输入
    stored_value: Option<f64>,        //计算结果
    current_operator: Option<String>, //计算操作符
    reset_display: bool,              // 是否清屏
}

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let state = use_state(|| CalculatorState {
        display: "0".to_string(),
        stored_value: None,
        current_operator: None,
        reset_display: false,
    });

    // 处理输入的字符
    let handle_number = {
        let state = state.clone();
        Callback::from(move |num: String| {
            //  安全地获取状态的深拷贝.
            // *state：利用 Deref trait 解引用 UseStateHandle<T>，得到 T 类型的值（例如 CalculatorState）。
            // state.clone() 克隆的是智能指针（UseStateHandle<T>），而非内部数据。由于 UseStateHandle<T> 的不可变性，无法直接修改其内部数据。
            let mut new_state = (*state).clone();
            // 重新清屏后显示
            if new_state.reset_display {
                new_state.display = num.clone();
                new_state.reset_display = false;
            } else if new_state.display == "0" {
                new_state.display = num.clone();
            } else {
                new_state.display.push_str(&num);
            }
            state.set(new_state);
        })
    };

    // 处理ac
    let handle_ac = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(CalculatorState {
                display: "0".to_string(),
                stored_value: None,
                current_operator: None,
                reset_display: false,
            });
        })
    };
    // 处理操作符
    let handle_operate = {
        let state = state.clone();
        Callback::from(move |op: String| {
            // 处理当前存储值和显示值的计算
            let mut new_state = (*state).clone();
            if let Some(stored_val) = new_state.stored_value {
                // 连续计算,当前有暂存值
                if let Some(stored_op) = new_state.current_operator {
                    if let Ok(curr) = new_state.display.parse::<f64>() {
                        let res = match stored_op.as_str() {
                            "+" => stored_val + curr,
                            "-" => stored_val - curr,
                            "*" => stored_val * curr,
                            "÷" => stored_val / curr,
                            _ => curr, // 默认值处理
                        };
                        new_state.display = format!("{}", res);
                    }
                }
            }
            // 将计算后的结果存储
            new_state.current_operator = Some(op);
            new_state.stored_value = new_state.display.parse().ok();
            new_state.reset_display = true; // TODO

            state.set(new_state);
        })
    };
    // 处理等号
    let handle_eq = {
        let state = state.clone();
        Callback::from(move |_| {
            // 处理当前存储值和显示值的计算
            let mut new_state = (*state).clone();
            if let (Some(stored_val), Some(stored_op)) =
                (new_state.stored_value, new_state.current_operator.clone())
            {
                // 连续计算,当前有暂存值
                if let Ok(curr) = new_state.display.parse::<f64>() {
                    let res = match stored_op.as_str() {
                        "+" => stored_val + curr,
                        "-" => stored_val - curr,
                        "*" => stored_val * curr,
                        "÷" => {
                            if curr != 0.0 {
                                stored_val / curr
                            } else {
                                f64::NAN
                            }
                        }
                        _ => curr, // 默认值处理
                    };
                    new_state.display = if res.is_nan() {
                        "Error".to_string()
                    } else {
                        format!("{}", res)
                    };
                    // 将计算后的结果存储
                    new_state.current_operator = None;
                    new_state.stored_value = Some(res);
                    new_state.reset_display = true; // TODO
                }
            }

            state.set(new_state);
        })
    };

    // 处理%
    let handle_percent = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            if let Ok(num) = new_state.display.parse::<f64>() {
                new_state.display = format!("{}", num / 100.0);
            }
            state.set(new_state);
        })
    };
    // 取正负
    let handle_negative = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            if let Ok(num) = new_state.display.parse::<f64>() {
                new_state.display = format!("{}", -num);
            }
            state.set(new_state);
        })
    };

    html! {
        <div class="calculator">
         <link rel="stylesheet" href="public/style.css" />
            <div class="display" id="display">{ &state.display }</div>
            <div class="buttons">
                <button class="button" onclick={handle_ac.clone()}>{"AC"}</button>
                <button class="button" onclick={handle_negative.clone()}>{"+/-"}</button>
                <button class="button" onclick={handle_percent.clone()}>{"%"}</button>
                <button class="button" onclick={handle_operate.clone().reform(|_| "÷".to_string())}>{"÷"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "7".to_string())} >{"7"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "8".to_string())} >{"8"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "9".to_string())}>{"9"}</button>
                <button class="button" onclick={handle_operate.clone().reform(|_| "*".to_string())}>{"×"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "4".to_string())}>{"4"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "5".to_string())}>{"5"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "6".to_string())}>{"6"}</button>
                <button class="button" onclick={handle_operate.clone().reform(|_| "-".to_string())}>{"-"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "1".to_string())}>{"1"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "2".to_string())}>{"2"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| "3".to_string())}>{"3"}</button>
                <button class="button" onclick={handle_operate.clone().reform(|_| "+".to_string())}>{"+"}</button>
                <button class="button zero" onclick={handle_number.clone().reform(|_| "0".to_string())}>{"0"}</button>
                <button class="button" onclick={handle_number.clone().reform(|_| ".".to_string())}>{"."}</button>
                <button class="button" onclick={handle_eq.clone()}>{"="}</button>
            </div>
        </div>
    }
}
