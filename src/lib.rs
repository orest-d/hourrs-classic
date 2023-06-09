mod model;
use crate::model::HoursData;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_free_icons::icons::io_icons::{
    IoArrowDownCircleOutline, IoArrowUpCircleOutline, IoPencilOutline, IoPlayCircleOutline,
    IoStopCircleOutline, IoTrashBinOutline, IoArrowBackCircleOutline, IoArrowForwardCircleOutline,
};
use dioxus_free_icons::Icon;
use dioxus_router::{use_route, use_router, Route, Router};
use model::{HoursRecord, Period};

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
    let show_login = use_state(cx, || false);
    let router = use_router(cx);

    cx.render(rsx! {
        h1 {
            title_text,
            button{
                class:"home",
                onclick: move |_event|{
                    router.navigate_to("/");
                },
                "Home"
            }
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
                                password.set("".to_string());
                            }
                        },
                    })
                }
                else{
                    rsx!(button{
                        class:"admin",
                        onclick: move |_event|{
                            show_login.set(true);
                            password.set("".to_string());
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
                                hours_data.save();
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
                                hours_data.save();
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
                                hours_data.save();
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
                    hours_data.save();
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

fn user_view<'a>(cx: Scope<'a, PeriodProps<'a>>) -> Element {
    let route = use_route(cx);
    let name = route.segment("name").unwrap_or("?");
    let hours_data = cx.props.hours_data;
    let mode = cx.props.mode;
    let year = cx.props.year;
    let month = cx.props.month;
    let period = use_state(cx, || Period::new(year, month));

    cx.render(rsx! {
        div{
            page_title{
                mode: mode,
                title_text: name.to_string(),
            },
            span{
                class:"period",
                "{period}"
            },
            span{
                class:"perdiodbutton",
                onclick: move |_event|{
                    let new_period = period.get().previous();
                    if true || new_period>=hours_data.read().dataframe.first_period(){
                        period.set(new_period);
                    }
                },
                Icon{
                    width: 24,
                    height: 24,
                    icon: IoArrowBackCircleOutline,
                }
            },
            span{
                class:"perdiodbutton",
                onclick: move |_event|{
                    let new_period = period.get().next();
                    if true || new_period<=Period::current() || new_period<=hours_data.read().dataframe.last_period(){
                        period.set(new_period);
                    }
                },
                Icon{
                    width: 24,
                    height: 24,
                    icon: IoArrowForwardCircleOutline,
                }
            },
            span{
                class:"e",
            },
            span{
                class:"a",
                "Hours worked:"
            },
            span{
                class:"b",
                "{hours_data.read().dataframe.status_for_period(name, period)}"
            },
            span{
                class:"e",
            },
            span{
                class:"a",
                "Total hours in {period}"
            },
            span{
                class:"b",
                "{hours_data.read().dataframe.hours_for_period(name, period):02}"
            },
            span{
                class:"e",
            },
            if hours_data.read().is_started(name){
                rsx!{button{
                    class:"menu",
                    onclick: move |_event|{
                        let mut hours_data = hours_data.write();
                        hours_data.end(name).unwrap_or_else(|e|{
                            println!("Error ending period: {}", e);
                        });
                    },
                    span{
                        class:"icon1",
                        Icon{
                            width: 20,
                            height: 20,
                            icon: IoStopCircleOutline,
                        }
                    }
                    "End"
                }}
            }else{
                rsx!{button{
                    class:"menu",
                    onclick: move |_event|{
                        let mut hours_data = hours_data.write();
                        hours_data.start(name).unwrap_or_else(|e|{
                            println!("Error starting period: {}", e);
                        });
                    },
                    span{
                        class:"icon1",
                        Icon{
                            width: 20,
                            height: 20,
                            icon: IoPlayCircleOutline,
                        }
                    }
                    "Start"
                }}
            },
            br{},
            period_overview{
                hours_data: hours_data,
                month: period.get().month,
                year: period.get().year,
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
pub struct PeriodEntryProps<'a> {
    pub hours_data: &'a UseRef<HoursData>,
    pub mode: &'a UseRef<Mode>,
    pub i: usize,
}

pub fn edit_period_entry<'a>(cx: Scope<'a, PeriodEntryProps<'a>>) -> Element {
    let edit_field = use_state(cx, || false);
    let mode = cx.props.mode;
    let hours_data = cx.props.hours_data;
    let i = cx.props.i;
    let record = hours_data.read().dataframe.data[i].clone();

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
            if mode.read().is_admin(){
                if *edit_field.get(){
                    let value = record.hours.clone();
                    rsx!{
                        input{
                            value: "{value}",
                            oninput: move |event|{
                                let mut hours_data = hours_data.write();
                                hours_data.dataframe.data[i].hours = event.value.to_string();
                            },
                            onkeypress: move |event|{
                                if event.key()==Key::Enter{
                                    edit_field.set(false);
                                    hours_data.read().save().unwrap_or_else(|e|{
                                        println!("Error editing field {}: {}", i, e);
                                    });
                                }
                            },
                        }
                    }
                }
                else{
                    rsx!{
                        span{
                            class:"d",
                            record.hours().to_string()
                        },
                        span{
                            class:"icon",
                            onclick: move |_event|{
                                edit_field.set(true);
                            },
                            Icon{
                                width: 16,
                                height: 16,
                                icon: IoPencilOutline,
                            },
                        }
                    }
                }
            }
            else{
                rsx!{
                    span{
                        class:"d",
                        record.hours().to_string()
                    },
                }
            }
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
    let mode = cx.props.mode;
    let month = cx.props.month;
    let year = cx.props.year;
    let index = hours_data
        .read()
        .dataframe
        .data
        .iter()
        .enumerate()
        .filter(|(_i, x)| (x.name == name) && (x.year == year) && (x.month == month))
        .map(|(i, _x)| i)
        .collect::<Vec<usize>>();

    cx.render(rsx! {
        div{
            div{
                class:"a",
                b{"Date"}
            },
            span{
                class:"b",
                b{"Start"}
            },
            span{
                class:"b",
                b{"End"}
            },
            span{
                class:"c",
                b{"Recorded Hours"}
            },
            span{
                class:"d",
                b{"Final Hours"}
            },
            span{
                class:"e"
            }
        }
        div{
            for i in index.iter(){
                edit_period_entry{
                    hours_data: hours_data,
                    mode: mode,
                    i: *i,
                }
                //period_entry{record: record.clone()}
            }
        }
    })
}

pub fn app(cx: Scope) -> Element {
    let hours_data = use_ref(cx, || {
        HoursData::load().unwrap_or_else(|e| {
            println!("Error loading data: {}", e);
            HoursData::default()
        })
    });
    let mode = use_ref(cx, || Mode::default());
    let _names = hours_data.read().names.clone();
    //let names = data.names.clone();
    let period = Period::current();
    cx.render(rsx! {
        style{
            include_str!("../src/style.css")
        },
        Router{
            /*
            ul{
                li{Link{to: "/names", "Names"}},
            }
            */
            //Route{to: "/names", show_names{names: names}},
            //Redirect{to: "/names"},
            Route{to: "/user/:name", user_view{hours_data: hours_data, mode: mode, year: period.year, month: period.month}},
            Route{to: "/names", users_page{hours_data: hours_data, mode: mode}},
            Route{to: "/", users_page{hours_data: hours_data, mode: mode}},
        }
    })
}
