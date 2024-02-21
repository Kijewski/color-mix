use std::num::NonZeroU8;
use std::rc::Rc;

use float_ord::FloatOrd;
use palette::color_difference::Wcag21RelativeContrast;
use palette::{
    Hsl, Hsluv, Hsv, Hwb, IntoColor, Lab, Lch, Lchuv, LinSrgb, Luv, Mix, Okhsl, Okhsv, Okhwb,
    Oklab, Oklch, Srgb, Xyz, Yxy,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, UseStateHandle};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let start = use_state(|| Rc::<str>::from("#003366"));
    let end = use_state(|| Rc::<str>::from("#99CC00"));
    let steps = use_state(|| Rc::<str>::from("12"));

    let input = |state: &UseStateHandle<Rc<str>>| {
        let state = state.clone();
        Callback::from(move |ev: InputEvent| {
            let value = ev
                .target()
                .unwrap_throw()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.set(Rc::from(value));
        })
    };
    let input_start = input(&start);
    let input_end = input(&end);
    let input_steps = input(&steps);

    let set = |from: &'static str, to: &'static str| {
        let start = start.clone();
        let end = end.clone();
        Callback::from(move |_| {
            start.set(from.into());
            end.set(to.into());
        })
    };

    let invert = {
        let start = start.clone();
        let end = end.clone();
        Callback::from(move |_| {
            let tmp = (*start).clone();
            start.set((*end).clone());
            end.set(tmp);
        })
    };

    let v_start: Option<Srgb<u8>> = start.parse().ok();
    let v_end: Option<Srgb<u8>> = end.parse().ok();
    let v_steps: Option<NonZeroU8> = steps.parse().ok();

    let mix = if let (Some(start), Some(end), Some(steps)) = (v_start, v_end, v_steps) {
        let start: Srgb = start.into_format();
        let end: Srgb = end.into_format();
        let factor = 1.0 / (steps.get() - 1).max(1) as f32;

        macro_rules! li {
            ($ty:ty, $text:expr) => {{
                (0..steps.get()).map(|idx| {
                    let start: $ty = start.into_color();
                    let end = end.into_color();
                    let value = start.mix(end, factor * idx as f32);
                    let value: Srgb = value.into_color();

                    let text_color = TEXT_COLORS
                        .iter()
                        .copied()
                        .max_by_key(|c| FloatOrd(c.relative_contrast(value)))
                        .unwrap_or_default();
                    let text_color: Srgb = text_color.into_color();
                    let text_color: Srgb<u8> = text_color.into_format();
                    let (tr, tg, tb) = text_color.into_components();

                    let background: Srgb<u8> = value.into_format();
                    let (br, bg, bb) = background.into_components();
                    let css = format!("#{br:02x}{bg:02x}{bb:02x}");

                    let style = format!(
                        "\
                        background-color: {css};\
                        color: #{tr:02x}{tg:02x}{tb:02x};\
                        display: inline-block;\
                        height: 1.2lh;\
                        width: 8em;\
                        text-align: center;\
                        font-weight: bold;"
                    );
                    let text = &css; // $text(value, &css);

                    html! {
                        <li>
                            <span {style}><tt>{text}</tt></span>
                        </li>
                    }
                })
            }};
        }

        macro_rules! degrees_x {
            ($ty:ty) => {{
                li!($ty, |value: $ty, css: &str| {
                    let (x, y, z) = value.into_components();
                    let x = (x.into_degrees() + 360.0).rem_euclid(360.0);
                    format!("{css} ({x:>5.1}°, {y:.3}, {z:.3})")
                })
            }};
        }

        macro_rules! degrees_z {
            ($ty:ty) => {{
                li!($ty, |value: $ty, css: &str| {
                    let (x, y, z) = value.into_components();
                    let z = (z.into_degrees() + 360.0).rem_euclid(360.0);
                    format!("{css} ({x:.3}°, {y:.3}, {z:>5.1})")
                })
            }};
        }

        macro_rules! linear {
            ($ty:ty) => {{
                li!($ty, |value: $ty, css: &str| {
                    let (x, y, z) = value.into_components();
                    format!("{css} ({x:.3}, {y:.3}, {z:.3})")
                })
            }};
        }

        let items = [
            (
                "rgb",
                "https://docs.rs/palette/^0.7.4/palette/type.Srgb.html",
                &mut linear!(Srgb) as &mut dyn Iterator<Item = Html>,
            ),
            (
                "lin. srgb",
                "https://docs.rs/palette/^0.7.4/palette/type.LinSrgb.html",
                &mut linear!(LinSrgb) as _,
            ),
            (
                "hsl",
                "https://docs.rs/palette/^0.7.4/palette/struct.Hsl.html",
                &mut degrees_x!(Hsl) as _,
            ),
            (
                "okhsl",
                "https://docs.rs/palette/^0.7.4/palette/struct.Okhsl.html",
                &mut degrees_x!(Okhsl) as _,
            ),
            (
                "hsluv",
                "https://docs.rs/palette/^0.7.4/palette/struct.Hsluv.html",
                &mut degrees_x!(Hsluv) as _,
            ),
            (
                "hsv",
                "https://docs.rs/palette/^0.7.4/palette/struct.Hsv.html",
                &mut degrees_x!(Hsv) as _,
            ),
            (
                "okhsv",
                "https://docs.rs/palette/^0.7.4/palette/struct.Okhsv.html",
                &mut degrees_x!(Okhsv) as _,
            ),
            (
                "hwb",
                "https://docs.rs/palette/^0.7.4/palette/struct.Hwb.html",
                &mut degrees_x!(Hwb) as _,
            ),
            (
                "okhwb",
                "https://docs.rs/palette/^0.7.4/palette/struct.Okhwb.html",
                &mut degrees_x!(Okhwb) as _,
            ),
            (
                "lab",
                "https://docs.rs/palette/^0.7.4/palette/struct.Lab.html",
                &mut linear!(Lab) as _,
            ),
            (
                "oklab",
                "https://docs.rs/palette/^0.7.4/palette/struct.Oklab.html",
                &mut linear!(Oklab) as _,
            ),
            (
                "lch",
                "https://docs.rs/palette/^0.7.4/palette/struct.Lch.html",
                &mut degrees_z!(Lch) as _,
            ),
            (
                "oklch",
                "https://docs.rs/palette/^0.7.4/palette/struct.Oklch.html",
                &mut degrees_z!(Oklch) as _,
            ),
            (
                "lchuv",
                "https://docs.rs/palette/^0.7.4/palette/struct.Lchuv.html",
                &mut degrees_z!(Lchuv) as _,
            ),
            (
                "luv",
                "https://docs.rs/palette/^0.7.4/palette/struct.Luv.html",
                &mut linear!(Luv) as _,
            ),
            (
                "xyz",
                "https://docs.rs/palette/^0.7.4/palette/struct.Xyz.html",
                &mut linear!(Xyz) as _,
            ),
            (
                "yxy",
                "https://docs.rs/palette/^0.7.4/palette/struct.Yxy.html",
                &mut linear!(Yxy) as _,
            ),
        ];

        Some(html! {
            <div style="\
                display: flex;\
                flex-wrap: wrap;\
                flex-direction: row;\
                justify-content: flex-start;\
                align-content: flex-start;\
                align-items: baseline;\
                gap: 0.5em 1em;
            ">
                {for items.into_iter().map(|(name, href, values)| {
                    html! {
                        <div style="\
                            width: max-content;\
                            min-width: max-content;\
                            display: inline-block;\
                        ">
                            <h3 style="text-align: center">
                                <a {href} target="_blank">{name}</a>
                            </h3>
                            <ol>{for values}</ol>
                        </div>
                    }
                })}
            </div>
        })
    } else {
        None
    };

    html! {
        <div>
            <p>
                <button type="button" onclick={set("#f03010", "#00c020")}>
                    {"red → green"}
                </button>
                {" "}
                <button type="button" onclick={set("#00c020", "#201080")}>
                    {"green → blue"}
                </button>
                {" "}
                <button type="button" onclick={set("#201080", "#e8f860")}>
                    {"blue → yellow"}
                </button>
                {" "}
                <button type="button" onclick={set("#e8f860", "#f03010")}>
                    {"yellow → red"}
                </button>
                {" "}
                <button type="button" onclick={set("#100408", "#f3f7ff")}>
                    {"reddish black → blueish white"}
                </button>
            </p>
            <p>
                <label>
                    {"Start: "}
                    <input
                        type="text"
                        pattern="^#[0-9a-fA-F]{6}$"
                        value={(*start).clone()}
                        oninput={input_start}
                        autocomplete="off"
                        spellcheck="false"
                        required=true
                    />
                </label>
            </p>
            <p>
                <label>
                    {"End: "}
                    <input
                        type="text"
                        pattern="^#[0-9a-fA-F]{6}$"
                        value={(*end).clone()}
                        oninput={input_end}
                        autocomplete="off"
                        spellcheck="false"
                        required=true
                    />
                </label>
                {" "}
                <button type="button" onclick={invert}>{"⇅"}</button>
            </p>
            <p>
                <label>
                    {"Steps: "}
                    <input
                        type="range"
                        min="3"
                        max="50"
                        value={(*steps).clone()}
                        oninput={input_steps.clone()}
                        autocomplete="off"
                        spellcheck="false"
                        required=true
                    />
                </label>
                {" "}
                <input
                    type="number"
                    min="3"
                    max="50"
                    value={(*steps).clone()}
                    oninput={input_steps}
                    autocomplete="off"
                    spellcheck="false"
                    required=true
                />
            </p>
            <>{mix}</>
        </div>
    }
}

const TEXT_COLORS: &[Srgb] = &[WHITE, BLACK];
const WHITE: Srgb = Srgb::new(0.96, 0.96, 0.96);
const BLACK: Srgb = Srgb::new(0.04, 0.04, 0.04);
