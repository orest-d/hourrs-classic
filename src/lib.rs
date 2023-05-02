mod model;
use crate::model::HoursData;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_free_icons::icons::io_icons::{
    IoArrowDownCircleOutline, IoArrowUpCircleOutline, IoTrashBinOutline,
};
use dioxus_free_icons::Icon;
use dioxus_router::{use_route, use_router, Link, Route, Router};
use model::HoursRecord;

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
                        class:"name",
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

#[derive(Default, Clone, Debug)]
pub struct Mode {
    login_time: Option<chrono::DateTime<chrono::Local>>,
}

impl Mode {
    pub fn is_admin(&self) -> bool {
        if let Some(t) = self.login_time {
            let now = chrono::Local::now();
            let duration = now - t;
            duration.num_seconds() < 60 * 10
        } else {
            false
        }
    }

    pub fn login(&mut self, password: &str) -> bool {
        if password == include_str!("../src/password.txt") {
            self.login_time = Some(chrono::Local::now());
            true
        } else {
            false
        }
    }

    pub fn logout(&mut self) {
        self.login_time = None;
    }
}

#[derive(Props)]
struct TitleProps<'a> {
    pub mode: &'a UseRef<Mode>,
    pub title_text: String,
}

fn page_title<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    let mode = cx.props.mode;
    let title_text = cx.props.title_text.clone();
    let password = use_state(cx, || "".to_string());
    let show_login= use_state(cx, || false);

    cx.render(rsx! {
        h1 {
            title_text,
            if mode.read().is_admin(){
                rsx!{
                    button{
                        class:"menu",
                        onclick: move |_event|{
                            mode.write().logout();
                        },
                        "Logout"
                    }
                }
            }
            else{
                if *show_login.get(){
                    rsx!(
                        input{
                        class: "password",
                        "type": "password",
                        value: "{password}",
                        oninput: move |event|{
                            password.set(event.value.clone());
                        },
                        onkeypress: move |event|{
                            if event.key()==Key::Enter{
                                mode.write().login(&password.get());
                                show_login.set(false);
                            }
                        },
                    })
                }
                else{
                    rsx!(button{
                        class:"admin",
                        onclick: move |_event|{
                            show_login.set(true);
                        },
                        "Admin"
                    })
                }
            },
        }
    })
}

#[derive(Props)]
struct HoursDataProps<'a> {
    pub hours_data: &'a UseRef<HoursData>,
    pub mode: &'a UseRef<Mode>,
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

fn users_page<'a>(cx: Scope<'a, HoursDataProps<'a>>) -> Element {
    let hours_data = cx.props.hours_data;
    let mode = cx.props.mode;
    let names = hours_data.read().names.clone();
    cx.render(rsx! {
        page_title{
            mode: mode,
            title_text: "Users".to_string(),
        },
        if mode.read().is_admin(){
            rsx!{
                edit_names{
                    hours_data: hours_data,
                    mode: mode,
                }
            }
        }
        else{
            rsx!{
                show_names{
                    names: names,
                }
            }
        },
    })
}

fn user_view<'a>(cx: Scope<'a, HoursDataProps<'a>>) -> Element {
    let route = use_route(cx);
    let name = route.segment("name").unwrap();
    let hours_data = cx.props.hours_data;
    let mode = cx.props.mode;

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
                mode: mode,
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
    pub mode: &'a UseRef<Mode>,
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
    let mode = use_ref(cx, || Mode::default());
    let names = hours_data.read().names.clone();
    //let names = data.names.clone();
    cx.render(rsx! {
        style{
            include_str!("../src/style.css")
        },
        Router{
            ul{
              li{Link{to: "/names", "Names"}},
            }
            //Route{to: "/names", show_names{names: names}},
            Route{to: "/user/:name", user_view{hours_data: hours_data, mode: mode}},
            Route{to: "/names", users_page{hours_data: hours_data, mode: mode}},
        }
    })
}
