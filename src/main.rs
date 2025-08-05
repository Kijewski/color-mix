use std::num::NonZeroU8;
use std::rc::Rc;

use float_ord::FloatOrd;
use palette::cam16::{
    Cam16FromUnclamped, Cam16IntoUnclamped, Cam16Jch, Cam16Jmh, Cam16Jsh, Cam16Qch, Cam16Qmh,
    Cam16Qsh, Parameters, StaticWp,
};
use palette::color_difference::Wcag21RelativeContrast;
use palette::white_point::D65;
use palette::{
    FromColor, Hsl, Hsluv, Hsv, Hwb, IntoColor, Lab, Lch, Lchuv, LinSrgb, Luv, Mix, Okhsl, Okhsv,
    Okhwb, Oklab, Oklch, Srgb, Xyz, Yxy,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{Callback, Html, InputEvent, UseStateHandle, function_component, html, use_state};

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
        let steps = steps.get();
        let factor = 1.0 / (steps - 1).max(1) as f32;

        let items = [
            (
                "rgb",
                "https://docs.rs/palette/0.7.6/palette/type.Srgb.html",
                &mut colors::<Srgb>(start, end, factor, steps) as &mut dyn Iterator<Item = Html>,
            ),
            (
                "lin. srgb",
                "https://docs.rs/palette/0.7.6/palette/type.LinSrgb.html",
                &mut colors::<LinSrgb>(start, end, factor, steps) as _,
            ),
            (
                "hsl",
                "https://docs.rs/palette/0.7.6/palette/struct.Hsl.html",
                &mut colors::<Hsl>(start, end, factor, steps) as _,
            ),
            (
                "okhsl",
                "https://docs.rs/palette/0.7.6/palette/struct.Okhsl.html",
                &mut colors::<Okhsl>(start, end, factor, steps) as _,
            ),
            (
                "hsluv",
                "https://docs.rs/palette/0.7.6/palette/struct.Hsluv.html",
                &mut colors::<Hsluv>(start, end, factor, steps) as _,
            ),
            (
                "hsv",
                "https://docs.rs/palette/0.7.6/palette/struct.Hsv.html",
                &mut colors::<Hsv>(start, end, factor, steps) as _,
            ),
            (
                "okhsv",
                "https://docs.rs/palette/0.7.6/palette/struct.Okhsv.html",
                &mut colors::<Okhsv>(start, end, factor, steps) as _,
            ),
            (
                "hwb",
                "https://docs.rs/palette/0.7.6/palette/struct.Hwb.html",
                &mut colors::<Hwb>(start, end, factor, steps) as _,
            ),
            (
                "okhwb",
                "https://docs.rs/palette/0.7.6/palette/struct.Okhwb.html",
                &mut colors::<Okhwb>(start, end, factor, steps) as _,
            ),
            (
                "lab",
                "https://docs.rs/palette/0.7.6/palette/struct.Lab.html",
                &mut colors::<Lab>(start, end, factor, steps) as _,
            ),
            (
                "oklab",
                "https://docs.rs/palette/0.7.6/palette/struct.Oklab.html",
                &mut colors::<Oklab>(start, end, factor, steps) as _,
            ),
            (
                "lch",
                "https://docs.rs/palette/0.7.6/palette/struct.Lch.html",
                &mut colors::<Lch>(start, end, factor, steps) as _,
            ),
            (
                "oklch",
                "https://docs.rs/palette/0.7.6/palette/struct.Oklch.html",
                &mut colors::<Oklch>(start, end, factor, steps) as _,
            ),
            (
                "lchuv",
                "https://docs.rs/palette/0.7.6/palette/struct.Lchuv.html",
                &mut colors::<Lchuv>(start, end, factor, steps) as _,
            ),
            (
                "luv",
                "https://docs.rs/palette/0.7.6/palette/struct.Luv.html",
                &mut colors::<Luv>(start, end, factor, steps) as _,
            ),
            (
                "xyz",
                "https://docs.rs/palette/0.7.6/palette/struct.Xyz.html",
                &mut colors::<Xyz>(start, end, factor, steps) as _,
            ),
            (
                "yxy",
                "https://docs.rs/palette/0.7.6/palette/struct.Yxy.html",
                &mut colors::<Yxy>(start, end, factor, steps) as _,
            ),
            (
                "Cam16Jch",
                "https://docs.rs/palette/0.7.6/palette/cam16/struct.Cam16Jch.html",
                &mut colors_cam16::<Cam16Jch<f32>>(start, end, factor, steps) as _,
            ),
            (
                "Cam16Jmh",
                "https://docs.rs/palette/0.7.6/palette/cam16/struct.Cam16Jmh.html",
                &mut colors_cam16::<Cam16Jmh<f32>>(start, end, factor, steps) as _,
            ),
            (
                "Cam16Jsh",
                "https://docs.rs/palette/0.7.6/palette/cam16/struct.Cam16Jsh.html",
                &mut colors_cam16::<Cam16Jsh<f32>>(start, end, factor, steps) as _,
            ),
            (
                "Cam16Qch",
                "https://docs.rs/palette/0.7.6/palette/cam16/struct.Cam16Qch.html",
                &mut colors_cam16::<Cam16Qch<f32>>(start, end, factor, steps) as _,
            ),
            (
                "Cam16Qmh",
                "https://docs.rs/palette/0.7.6/palette/cam16/struct.Cam16Qmh.html",
                &mut colors_cam16::<Cam16Qmh<f32>>(start, end, factor, steps) as _,
            ),
            (
                "Cam16Qsh",
                "https://docs.rs/palette/0.7.6/palette/cam16/struct.Cam16Qsh.html",
                &mut colors_cam16::<Cam16Qsh<f32>>(start, end, factor, steps) as _,
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

fn colors<T>(start: Srgb, end: Srgb, factor: f32, steps: u8) -> impl Iterator<Item = Html>
where
    Srgb: IntoColor<T>,
    T: IntoColor<Srgb> + Mix<Scalar = f32> + Copy,
{
    let start: T = start.into_color();
    let end: T = end.into_color();
    (0..steps).map(move |idx| color_row(start.mix(end, factor * idx as f32).into_color()))
}

fn colors_cam16<T>(start: Srgb, end: Srgb, factor: f32, steps: u8) -> impl Iterator<Item = Html>
where
    T: Mix<Scalar = f32>
        + Cam16FromUnclamped<StaticWp<D65>, Xyz, Scalar = f32>
        + Cam16IntoUnclamped<StaticWp<D65>, Xyz, Scalar = f32>
        + Copy,
{
    let parameters = Parameters::default_static_wp(40.0).bake();
    let start = T::cam16_from_unclamped(start.into_color(), parameters);
    let end = T::cam16_from_unclamped(end.into_color(), parameters);
    (0..steps).map(move |idx| {
        color_row(
            start
                .mix(end, factor * idx as f32)
                .cam16_into_unclamped(parameters)
                .into_color(),
        )
    })
}

fn color_row(value: Srgb) -> Html {
    let text_color = TEXT_COLORS
        .iter()
        .copied()
        .max_by_key(|c| FloatOrd(c.relative_contrast(value)))
        .unwrap_or_default();
    let text_color = Srgb::from_color(text_color);
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

    html! {
        <li>
            <span {style}><tt>{css}</tt></span>
        </li>
    }
}

const TEXT_COLORS: &[Srgb] = &[WHITE, BLACK];
const WHITE: Srgb = Srgb::new(0.96, 0.96, 0.96);
const BLACK: Srgb = Srgb::new(0.04, 0.04, 0.04);
