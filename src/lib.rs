use leptos::*;
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
mod interpreter;
use interpreter::{Color, PrimaryColor, Expr, build_expr_from_colors, evaluate, to_colourful,extract_colors};
use crate::PrimaryColor::{Red, Blue, Yellow};
#[derive(Serialize)]
struct EvalRequest {
    colors: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct EvalResponse {
    topColors: Vec<String>,
}

async fn evaluate_colors(colors: Vec<String>) -> Result<Vec<String>, gloo_net::Error> {
    let response = Request::post("http://localhost:8080/evaluate")
        .header("Content-Type", "application/json")
        .json(&EvalRequest { colors })?
        .send()
        .await?;

    let result: EvalResponse = response.json().await?;
    Ok(result.topColors)
}

#[derive(Clone, PartialEq)]
struct BoxItem {
    id: usize,
    color: Color,
}
#[component]
fn App() -> impl IntoView {
    let (text, set_text) = create_signal(String::new());
    let (color, set_color) = create_signal(Color::Primary(Red));
    let (boxes, set_boxes) = create_signal(Vec::<BoxItem>::new());
    let next_id = create_rw_signal(0);
    let (show_evaluation, set_show_evaluation) = create_signal(false);
    let (is_dark, set_is_dark) = create_signal(false);


    let (colors, _set_colors) = create_signal(vec![
    Color::Primary(Red),
    Color::Primary(Blue),
    Color::Primary(Yellow),
    Color::Orange,
    Color::Green,
    Color::Purple,
    Color::Pink,
    Color::Cyan,
    Color::Violet,
    Color::Lime,
    Color::Teal,
]);

let (evaluation_result, set_evaluation_result) = create_signal(Vec::<Color>::new());
let (text_eval_result, set_text_eval_result) = create_signal(Vec::<Color>::new());
let color_buttons = {
    let color_list = colors.get().clone();
    color_list.into_iter().map(|c| {
        let c_clone = c.clone(); // clone once for reuse
        let is_selected = move || color.get() == c_clone;
        let button_style = {
            let c_clone = c.clone(); // another clone for style closure
            move || {
                let bg = to_css_color(&c_clone);
                if is_selected() {
                    format!(
                        "margin-right: 1.5rem; font-size: 1.5rem; background-color: {}; color: white; border: 1px solid black;",
                        bg
                    )
                } else {
                    "margin-right: 1.5rem;  font-size: 1.5rem; background-color: lightgray; color: black; border: 1px solid gray;".to_string()
                }
            }
        };
        let c_clone_for_click = c.clone(); // clone for click handler
        view! {
            <button
                style=button_style
                on:click=move |_| set_color.set(c_clone_for_click.clone())
            >
                {format!("{}", c)}
            </button>
        }
    }).collect::<Vec<_>>()
};

    view! {
        <main style={move || {
        if is_dark.get() {
            "background-color: #121212; color: #f0f0f0; padding: 3rem;"
        } else {
            "background: linear-gradient(to right, #fdfbfb, #ebedee); color: #333; padding: 3rem;"
        }
    }}
    >

               <h1 style="font-size: 3rem; margin-bottom: 1rem; text-align: center;">
        <span style="color: #e63946;">"C"</span>
        <span style="color: #f1a208;">"o"</span>
        <span style="color: #2a9d8f;">"l"</span>
        <span style="color: #457b9d;">"o"</span>
        <span style="color: #9d4edd;">"u"</span>
        <span style="color: #ff006e;">"r"</span>
        <span style="color: #06d6a0;">"f"</span>
        <span style="color: #f72585;">"u"</span>
        <span style="color: #3a0ca3;">"l"</span>
        " Playground 🎨"
    </h1>

<p style={move || {
    if is_dark.get() {
        "margin: 0 auto 2rem; max-width: 800px; color: #ccc; text-align: center;"
    } else {
        "margin: 0 auto 2rem; max-width: 800px; color: #333; text-align: center;"
    }
}}>
    "Colourful is an esoteric/satirical functional programming language where colours are used as combinators. Inspired by combinatory logic, it lets you build expressions using colour blocks. Evaluation happens right to left and bottom to top. This playground lets you experiment with expressions, see how they reduce, and explore the language interactively."
</p>

        <button on:click=move |_| set_is_dark.update(|v| *v = !*v)
            style="margin-bottom: 1rem; padding: 0.5rem 1rem; font-size: 1rem;">
        {move || if is_dark.get() { "☀️ Light Mode" } else { "🌙 Dark Mode" }}
    </button>



            <div style="margin-bottom: 1rem;">
               {color_buttons}


<button 
style="font-size: 1.5rem; padding: 0.5rem 1rem; margin-right: 1.5rem;"
on:click=move |_| {
     

    let new_box = BoxItem {
    id: next_id.get(),
    color: color.get(),
};
    next_id.update(|id| *id += 1);
    set_boxes.update(|b| b.push(new_box));
}>
    "Add Box"
</button>


<button 
 style="font-size: 1.5rem; padding: 0.5rem 1rem; margin-right: 1.5rem;"
on:click=move |_| {
    

    
let colors = boxes.get().iter().map(|b| b.color.clone()).collect::<Vec<_>>();
if !colors.is_empty() {
    let expr = build_expr_from_colors(&colors);
    let result = extract_colors(&evaluate(expr)).into_iter().rev().collect();
    set_evaluation_result.set(result);
}
}>
    "Evaluate"
</button>

                
            </div>

            <div style="display: flex; gap: 1rem; font-size: 1.5rem; flex-wrap: wrap;">
                <For
    each=move || boxes.get().clone()
    key=|item| item.id
    children=move |item| {
        let remove_box = {
            let id = item.id;
            move |_| {
                set_boxes.update(|b| b.retain(|x| x.id != id));
            }
        };
        
        view! {
            <div style=format!("position: relative; width: 100px; height: 100px; background-color: {};", to_css_color(&item.color))>
                <button
                    on:click=remove_box
                    style="
                        position: absolute;
                        top: 2px;
                        right: 4px;
                        background: white;
                        border: none;
                        border-radius: 50%;
                        width: 20px;
                        height: 20px;
                        cursor: pointer;
                        font-weight: bold;
                    "
                >"×"</button>
            </div>
        }
    }
/>
            </div>

{move || {
    let result_colors = evaluation_result.get();
    (!result_colors.is_empty()).then(|| {
        view! {
            <div style="margin-top: 2rem;">
                <h3>"Evaluation Result:"</h3>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                    {result_colors.iter().map(|c| {
                        let css = to_css_color(c);
                        view! {
                            <div style=format!("width: 100px; height: 100px; background-color: {}; border: 2px solid black;", css)>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        }
    })
}}

            <div style="display: flex; align-items: flex-start; gap: 1rem; margin-top: 1rem; margin-bottom: 1rem;">
                <textarea
                    style="flex: 1; max-width: 800px; height: 8rem; font-size: 1.2rem; padding: 0.5rem;"
                    prop:value=text
                    on:input=move |e| set_text.set(event_target_value(&e))
                />
                <button
                    style="font-size: 1.2rem; padding: 0.5rem 1rem; white-space: nowrap;"
                    on:click=move |_| {
                        let colors: Vec<Color> = text.get()
                            .split(|c: char| !c.is_alphabetic())
                            .filter_map(|word| color_from_name(word))
                            .collect();
                        if !colors.is_empty() {
                            let expr = build_expr_from_colors(&colors);
                            let result = extract_colors(&evaluate(expr)).into_iter().rev().collect();
                            set_text_eval_result.set(result);
                        }
                    }
                >
                    "Evaluate Text"
                </button>
            </div>
            {move || render_colored_text(text.get())}
            {move || {
                let colors = text_eval_result.get();
                (!colors.is_empty()).then(|| view! {
                    <div style="margin-top: 1rem;">
                    <h3>"Text Evaluation Result:"</h3>
                    <p style="font-size: 1.2rem;">
                        {colors.iter().map(|c| {
                            let css = to_css_color(c);
                            let name = format!("{} ", c);
                            view! { <span style=format!("color: {};", css)>{name}</span> }
                        }).collect::<Vec<_>>()}
                    </p>
                    </div>
                })
            }}

<div style="margin-top: 3rem;">
        <h3>"Colour Definitions"</h3>
       <ul style="line-height: 2;">
    <li><strong style="color: yellow;">Yellow</strong>: Identity function; returns its argument unchanged.</li>
    <li><strong style="color: red;">Red</strong>: Returns the first argument, discarding the second.</li>
    <li><strong style="color: blue;">Blue</strong>: Applies the result of applying the first argument to the third to the result of applying the second to the third.</li>
    <li><strong style="color: orange;">Orange</strong>: Red applied to Yellow.</li>
    <li><strong style="color: green;">Green</strong>: Blue applied to Yellow.</li>
    <li><strong style="color: purple;">Purple</strong>: Red applied to Blue.</li>
    <li><strong style="color: pink;">Pink</strong>: Red applied to Red.</li>
    <li><strong style="color: cyan;">Cyan</strong>: Blue applied to Blue.</li>
    <li><strong style="color: violet;">Violet</strong>: Blue applied to Red.</li>
    <li><strong style="color: lime;">Lime</strong>: Blue applied to Yellow and Yellow.</li>
    <li><strong style="color: teal;">Teal</strong>: Blue applied to Yellow and Blue.</li>
</ul>

    </div>

 // 🌐 GitHub Links Section
        <div style="margin-top: 3rem;">
            <h3>"Learn More About Colourful:"</h3>
            <p>
                <a href="https://github.com/AR2202/Colourful" target="_blank" style="color: blue; text-decoration: underline;">
                    "Language Specification & Source Code"
                </a>
            </p>
            <p>
                <a href="https://github.com/AR2202/Colourful/blob/main/Documentation.md" target="_blank" style="color: blue; text-decoration: underline;">
                    "Documentation"
                </a>
            </p>
        </div>


        </main>
    }
}


#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
fn color_from_name(name: &str) -> Option<Color> {
    match name.to_lowercase().as_str() {
        "red" => Some(Color::Primary(Red)),
        "blue" => Some(Color::Primary(Blue)),
        "yellow" => Some(Color::Primary(Yellow)),
        "orange" => Some(Color::Orange),
        "green" => Some(Color::Green),
        "purple" => Some(Color::Purple),
        "pink" => Some(Color::Pink),
        "cyan" => Some(Color::Cyan),
        "violet" => Some(Color::Violet),
        "lime" => Some(Color::Lime),
        "teal" => Some(Color::Teal),
        _ => None,
    }
}

fn render_colored_text(text: String) -> impl IntoView {
    const COLOR_NAMES: &[(&str, &str)] = &[
        ("red", "red"), ("blue", "blue"), ("yellow", "yellow"),
        ("orange", "orange"), ("green", "green"), ("purple", "purple"),
        ("pink", "pink"), ("cyan", "cyan"), ("violet", "violet"),
        ("lime", "lime"), ("teal", "teal"),
    ];

    let mut spans: Vec<leptos::View> = Vec::new();
    let mut current = String::new();
    let mut in_word = false;

    for ch in text.chars() {
        let is_alpha = ch.is_alphabetic();
        if is_alpha != in_word {
            if !current.is_empty() {
                let chunk = current.clone();
                let style = if in_word {
                    let lower = chunk.to_lowercase();
                    COLOR_NAMES.iter()
                        .find(|(name, _)| *name == lower)
                        .map(|(_, css)| format!("color: {};", css))
                        .unwrap_or_else(|| "color: gray;".to_string())
                } else {
                    "color: gray;".to_string()
                };
                spans.push(view! { <span style=style>{chunk}</span> }.into_view());
                current = String::new();
            }
            in_word = is_alpha;
        }
        current.push(ch);
    }
    if !current.is_empty() {
        let chunk = current.clone();
        let style = if in_word {
            let lower = chunk.to_lowercase();
            COLOR_NAMES.iter()
                .find(|(name, _)| *name == lower)
                .map(|(_, css)| format!("color: {};", css))
                .unwrap_or_else(|| "color: gray;".to_string())
        } else {
            "color: gray;".to_string()
        };
        spans.push(view! { <span style=style>{chunk}</span> }.into_view());
    }

    view! { <p style="white-space: pre-wrap;">{spans}</p> }
}

fn to_css_color(color: &Color) -> &'static str {
    match color {
        Color::Primary(Red) => "red",
        Color::Primary(Blue) => "blue",
        Color::Primary(Yellow) => "yellow",
        Color::Orange => "orange",
        Color::Green => "green",
        Color::Purple => "purple",
        Color::Pink => "pink",
        Color::Cyan => "cyan",
        Color::Violet => "violet",
        Color::Lime => "lime",
        Color::Teal => "teal",
    }
}
fn render_color_boxes(colors: Vec<Color>) -> impl IntoView {
    view! {
        <div style="display: flex; gap: 1rem; flex-wrap: wrap; margin-top: 2rem;">
            {colors.into_iter().map(|c| {
                let css = to_css_color(&c);
                view! {
                    <div style=format!("width: 100px; height: 100px; background-color: {}; border: 2px solid black;", css)>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
