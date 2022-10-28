#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const TEST_SOURCE: &str = r"type TestAlias = Int
type TestAliasDupe = Int
type TestAliasString = String
type TestVariable String = Maybe String
type TestList = [Char]
type TestNestedList = [[Char]]
type TestTuple = (String, Int)
type TestTupleNested = (String, (Int, Int))
type TestUnit = ()

type TestFunc = Int -> Int
type TestFuncDupe = Int -> Int
type TestFuncString = String -> String
type TestFuncTrip = Int -> String -> Int
type TestFunTuple = (Int, Int) -> String -> (String, (Int, Int))

type TestVar a = a -> a
type TestVarMixed a = a -> Int
type TestVarDiff a b = a -> b
type TestVarTuple a b = (a, a) -> b -> (b, (a, a))";

const TEST_TARGET: &str = r"Int -> Int";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        source_code: TEST_SOURCE.to_string(),
        // target_type: String::new(),
        target_type: TEST_TARGET.to_string(),
        aliases: vec![],
    }
}

// ------ ------
//     Model
// ------ ------

type Source = String;
type Target = String;

#[derive(Debug, Clone)]
struct Model {
    source_code: Source,
    target_type: Target,
    aliases: Vec<Alias>,
}

impl Model {
    fn to_request(&self) -> RequestAlias {
        RequestAlias {
            target_type: self.target_type.clone(),
            source: self.source_code.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Alias {
    matched: String,
    replaced_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestAlias {
    pub target_type: Target,
    pub source: Source,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMatches {
    echo_request: RequestAlias,
    matches: Vec<Match>,
}

impl ResponseMatches {
    fn to_aliases(self) -> Vec<Alias> {
        self.matches
            .iter()
            .map(|m| Alias {
                replaced_type: m.replaced_type.to_string(),
                matched: m.matched.to_string(),
            })
            .collect()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Match {
    matched: String,
    location: [[usize; 2]; 2],
    variable_map: HashMap<String, String>,
    replaced_type: String,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    GotAliases(fetch::Result<ResponseMatches>),
    Submit,
    UpdateSource(Source),
    UpdateTarget(Target),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Submit => {
            orders.skip().perform_cmd({
                let message = model.to_request();
                log!(model);
                async { Msg::GotAliases(fetch_aliases(message).await) }
            });
        }

        Msg::GotAliases(Ok(response)) => model.aliases = response.to_aliases(),

        Msg::GotAliases(Err(fetch_err)) => {
            log!(fetch_err);
            orders.skip();
        }

        Msg::UpdateSource(source) => model.source_code = source,

        Msg::UpdateTarget(target) => model.target_type = target,
    }
}

async fn fetch_aliases(message: RequestAlias) -> fetch::Result<ResponseMatches> {
    const BACKEND_URL: &str = "http://127.0.0.1:3000/api";

    let request = Request::new(BACKEND_URL)
        .method(Method::Post)
        .json(&message)?;

    let response = fetch(request).await.expect("Error fetching from backend:");

    response.json().await
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    section![
        C!["section"],
        h2![C!["title", "is-size-2"], "Type alias resolver"],
        code_area(),
        target_type_input(),
        alias_display(model),
    ]
}

fn code_area() -> Node<Msg> {
    div![
        C!["field"],
        div![
            C!["control"],
            textarea![
                C!["textarea", "is-family-monospace"],
                attrs! {
                    At::Placeholder => "Source code goes here",
                    At::Rows => 20,
                    At::SpellCheck => false,
                },
                input_ev(Ev::Input, Msg::UpdateSource),
                TEST_SOURCE
            ]
        ]
    ]
}

fn target_type_input() -> Node<Msg> {
    div![
        C!["field", "is-horizontal"],
        div![
            C!["field-label", "is-normal"],
            div![C!["label"], "Target type alias:"]
        ],
        div![
            C!["field-body"],
            div![
                C!["field", "has-addons"],
                div![
                    C!["control", "is-expanded"],
                    input![
                        C!["input", "is-family-monospace"],
                        input_ev(Ev::Input, Msg::UpdateTarget),
                        attrs! {
                            At::Type => "text",
                            At::Placeholder => "Int -> Int",
                        },
                    ]
                ],
                div![
                    C!["control"],
                    button![
                        C!["button", "is-info"],
                        ev(Ev::Click, |_| Msg::Submit),
                        "Submit"
                    ]
                ]
            ]
        ]
    ]
}

fn alias_display(model: &Model) -> Node<Msg> {
    ul![
        C!["content", "is-family-monospace"],
        model.aliases.iter().map(|a| li![a.replaced_type.clone()])
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
// #[wasm_bindgen(start)]
pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
