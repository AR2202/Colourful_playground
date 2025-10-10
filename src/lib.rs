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
        <main style="padding: 2rem; font-family: sans-serif;font-size: 1.5rem;">
           <h1 style="font-size: 2.5rem; margin-bottom: 0.5rem;">"Colourful Playground"</h1>
        <p style="margin-bottom: 2rem; max-width: 800px;">
            "Colourful is an esoteric/satirical functional programming language where colours are used as combinators. Inspired by combinatory logic, it lets you build expressions using colour blocks. Evaluation happens right to left and bottom to top. This playground lets you experiment with expressions, see how they reduce, and explore the language interactively."
        </p>


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

            <p style=move || format!("color: {};", color.get())>{text}</p>

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
<div style="margin-top: 3rem;">
        <h3>"Colour Definitions"</h3>
        <ul style="line-height: 2;">
       <li><strong>Yellow</strong>: Identity function; returns its argument unchanged.</li>

               <li><strong>Red</strong>: Returns the first argument, discarding the second.</li>

            <li><strong>Blue</strong>: Applies the first argument to both the second and third.</li>
            
            <li><strong>Orange</strong>: Red applied to Yellow.</li>
            <li><strong>Green</strong>: Blue Applied to Yellow.</li>
            <li><strong>Purple</strong>: Red applied to Blue.</li>
            <li><strong>Pink</strong>: Red applied to Red.</li>
            <li><strong>Cyan</strong>: Blue applied to Blue.</li>
            <li><strong>Violet</strong>: Blue applied to Red.</li>
            <li><strong>Lime</strong>: Blue applied to Yellow and Yellow.</li>
            <li><strong>Teal</strong>: Blue applied to Yellow and Blue.</li>
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
