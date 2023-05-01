mod model;
use crate::model::HoursData;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_router::{use_route, use_router, Link, Route, Router};
use model::HoursRecord;
use dioxus_free_icons::icons::io_icons::{IoArrowDownCircleOutline, IoArrowUpCircleOutline, IoTrashBinOutline};
use dioxus_free_icons::Icon;


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
                        onclick: move |_event|{
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
    let new_name = use_state(cx, || "".to_string());
    cx.render(rsx! {
        ul{
            for (i,name) in names.into_iter().enumerate(){
                p {
                    button{
                        class:"name",
                        onclick: move |_event|{
                            let path = format!("/user/{}", name);
                            dbg!(&path);
                            router.navigate_to(&path);
                        },
                        "{name}"
                    },
                    span{
                        onclick: move |_event|{
                            let mut hours_data = hours_data.write();
                            if hours_data.names.len()>1 && i>0{
                                let x = hours_data.names.remove(i);
                                hours_data.names.insert(i-1, x);
                            }
                        },
                        class:"icon",
                        Icon{
                            width: 40,
                            height: 40,
                            //fill: "white",
                            icon: IoArrowUpCircleOutline,
                        }
                    },
                    span{
                        onclick: move |_event|{
                            let mut hours_data = hours_data.write();
                            if hours_data.names.len()>1 && i<hours_data.names.len()-1{
                                let x = hours_data.names.remove(i);
                                hours_data.names.insert(i+1, x);
                            }
                        },
                        class:"icon",
                        Icon{
                            width: 40,
                            height: 40,
                            icon: IoArrowDownCircleOutline,
                        }
                    },
                    span{
                        onclick: move |_event|{
                            let mut hours_data = hours_data.write();
                            if !hours_data.names.is_empty(){
                                hours_data.names.remove(i);
                            }
                        },
                        class:"icon",
                        Icon{
                            width: 40,
                            height: 40,
                            icon: IoTrashBinOutline,
                        }
                    }
                }
            }
        },
        input{
            value: "{new_name}",
            oninput: move |event|{
                let _hours_data = hours_data.write();
                new_name.set(event.value.clone());
            },
            onkeypress: move |event|{
                if event.key()==Key::Enter{
                    let mut hours_data = hours_data.write();
                    hours_data.names.push(new_name.get().clone());
                    new_name.set("".to_string());
                }
            },

        },
    })
}

fn user_view<'a>(cx: Scope<'a, HoursDataProps<'a>>) -> Element {
    let route = use_route(cx);
    let name = route.segment("name").unwrap();
    let hours_data = cx.props.hours_data;

    cx.render(rsx! {
        div{
            h1{
                "{name}"
            },
            if hours_data.read().is_started(name){
                rsx!{button{
                    class:"menu",
                    onclick: move |_event|{
                        let mut hours_data = hours_data.write();
                        hours_data.end(name);
                    },
                    "End"
                }}
            }else{
                rsx!{button{
                    class:"menu",
                    onclick: move |_event|{
                        let mut hours_data = hours_data.write();
                        hours_data.start(name);
                    },
                    "Start"
                }}
            },
            period_overview{
                hours_data: hours_data,
                month: 4,
                year: 2023,
            }
        }

    })
}

#[inline_props]
pub fn period_entry(cx: Scope, record: HoursRecord) -> Element {
    cx.render(rsx! {
        div{
            div{
                class:"a",
                record.date()
            },
            span{
                class:"b",
                record.start_time()
            },
            span{
                class:"b",
                record.end_time()
            },
            span{
                class:"c",
                record.original_hours()   
            }
            span{
                class:"d",
                record.hours.to_string()
            },            
            span{
                class:"e"
            }
        }
    })
}

#[derive(Props)]
pub struct PeriodProps<'a> {
    pub hours_data: &'a UseRef<HoursData>,
    pub month: u32,
    pub year: i32,
}

fn period_overview<'a>(cx: Scope<'a, PeriodProps<'a>>) -> Element {
    let route = use_route(cx);
    let name = route.segment("name").unwrap();
    let hours_data = cx.props.hours_data;
    let month = cx.props.month;
    let year = cx.props.year;

    cx.render(rsx! {
        div{
            for record in hours_data.read().dataframe.data.iter().filter(
                |x| {(x.name==name) && (x.year == year) && (x.month == month)}){
                period_entry{record: record.clone()}
            }
        }
    })
}

pub fn app(cx: Scope) -> Element {
    let hours_data = use_ref(cx, || HoursData::from_store(".").unwrap());
    let names = hours_data.read().names.clone();
    //let names = data.names.clone();
    cx.render(rsx! {
        style{
            include_str!("../src/style.css")
        },
        h1 {
            "Hello, world!"
        },
        Router{
            ul{
              li{Link{to: "/names", "Names"}},
              li{Link{to: "/admin/names", "Edit names"}},
            }
            Route{to: "/names", show_names{names: names}},
            Route{to: "/user/:name", user_view{hours_data: hours_data}},
            Route{to: "/admin/names", edit_names{hours_data: hours_data}},
        }
    })
}
