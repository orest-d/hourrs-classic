mod model;
use crate::model::HoursData;
use dioxus::prelude::*;
use dioxus_router::{use_route, use_router, Link, Route, Router};

#[derive(PartialEq, Props)]
struct Names {
    pub names: Vec<String>,
}

fn show_names(cx: Scope<Names>) -> Element {
    let router = use_router(cx);
    cx.render(rsx! {
        ul{
            for name in cx.props.names.iter(){
                p{
                    button{
                        onclick: move |event|{
                            let path = format!("/user/{}", name);
                            dbg!(&path);
                            router.navigate_to(&path);
                        },
                        "{name}"
                    }
                }
            }
        }
    })
}

#[derive(Props)]
struct HoursDataProps<'a> {
    pub hours_data: &'a UseRef<HoursData>,
}


fn edit_names<'a>(cx: Scope<'a, HoursDataProps<'a>>) -> Element {
    let router = use_router(cx);
    let hours_data = cx.props.hours_data;
    let names = hours_data.read().names.clone();
    cx.render(rsx! {
        ul{
            for name in names.into_iter(){
                p {
                    button{
                        onclick: move |event|{
                            let path = format!("/user/{}", name);
                            dbg!(&path);
                            router.navigate_to(&path);
                        },
                        "{name}"
                    },
                    button{
                        "up"
                    }
                    button{
                        "down"
                    }

                }
            }
        }
    })
}

pub fn user_view(cx: Scope) -> Element {
    let route = use_route(cx);
    let name = route.segment("name").unwrap();

    cx.render(rsx! {
        div{
            h1{
                "{name}"
            }
        }
    })
}

pub fn app(cx: Scope) -> Element {
    let hours_data = use_ref(cx, || {
        HoursData::from_store("/home/orest/PycharmProjects/hours3/app/data").unwrap()
    });
    let data = HoursData::from_store("/home/orest/PycharmProjects/hours3/app/data").unwrap();

    //let names = data.names.clone();
    cx.render(rsx! {
        style{
            include_str!("../src/style.css")
        },
        h1 {
            "Hello, world!"
        },
        Router{
            Link{to: "/names", "Names"},
            Link{to: "/names", "Names"},
            Route{to: "/names", show_names{names: data.names.clone()}},
            Route{to: "/user/:name", user_view{}},
            Route{to: "/admin/names", edit_names{hours_data: hours_data}},
        }
    })
}
