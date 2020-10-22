// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

use log::debug;
use strum::IntoEnumIterator;
use yew::{
    events::KeyboardEvent,
    format::Json,
    html,
    services::storage::{ Area, StorageService },
    web_sys::HtmlInputElement as InputElement,
    Classes,
    Component,
    ComponentLink,
    Html,
    InputData,
    NodeRef,
    ShouldRender
};

use crate::{
    entry::Entry,
    filter::Filter,
    message::Message,
    state::State,
    KEY
};

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    focus_ref: NodeRef,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("Unable to start the storage service");

        let entries = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Vec::new()
            }
        };

        let state = State {
            entries,
            filter: Filter::All,
            value: String::new(),
            edit_value: String::new(),
        };

        let focus_ref = NodeRef::default();

        debug!("App component created");

        Self {
            link,
            storage,
            state,
            focus_ref,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::Add => {
                let description = self.state.value.trim();

                if !description.is_empty() {
                    let entry = Entry {
                        description: description.to_string(),
                        completed: false,
                        editing: false,
                    };

                    self.state.entries.push(entry);
                }

                self.state.value = String::new();
            },
            Message::Edit(idx) => {
                let editing = self.state.edit_value.trim().to_string();
                self.state.complete_edit(idx, editing);
                self.state.edit_value = String::new();
            },
            Message::Update(val) => {
                debug!("Input: {}", val);
                self.state.value = val;
            },
            Message::UpdateEdit(val) => {
                debug!("Input: {}", val);
                self.state.edit_value = val;
            },
            Message::Remove(idx) => self.state.remove(idx),
            Message::SetFilter(filter) => self.state.filter = filter,
            Message::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.clear_all_edit();
                self.state.toggle_edit(idx);
            },
            Message::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            },
            Message::Toggle(idx) => self.state.toggle(idx),
            Message::ClearCompleted => self.state.clear_completed(),
            Message::Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            }
        }

        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let is_hidden = if self.state.entries.is_empty() {
            "is-hidden"
        } else {
            ""
        };

        let view = include_str!("views/main.html");

        html!{
            <>
                <style>
                    {
                        "
                            .navbar > .navbar-brand > #brand{
                                font-family: 'Inconsolata', monospace;
                            }
                            .is-hidden {
                                visibility: hidden;
                            }
                        "
                    }
                </style>
                <nav class="navbar is-light"
                    role="navigation"
                    aria-label="main navigation"
                >
                    <div class="navbar-brand">
                        <a class="navbar-item" id="brand" href="#">
                            { "Yew • Todo" }
                        </a>

                        <a role="button"
                            class="navbar-burger burger"
                            aria-label="menu"
                            aria-expanded="false"
                            data-target="navbar-main"
                        >
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                        </a>
                    </div>

                    <div id="navbar-main" class="navbar-menu">
                        <div class="navbar-start">
                        // A navbar would go here
                        </div>
                    </div>
                </nav>
                <main role="main">
                    <div class="columns">
                        <div class="column is-three-fifths is-offset-one-fifth">
                            <section class="section" id="todo-app">
                                <div class="container">
                                    <div class="card">
                                        <header class="card-header">
                                            <p class="card-header-title">{ "Todos" }</p>
                                        </header>
                                        <div class="card-content">
                                            { self.view_input() }
                                        </div>
                                        <footer class="card-footer">
                                            { for Filter::iter().map(|filter| self.view_filter(filter)) }
                                            <a class="card-footer-item"
                                                id="toggle-all"
                                                // checked=self.state.is_all_completed()
                                                onclick=self.link.callback(|_| Message::ToggleAll)
                                            >
                                                { "Toggle All"}
                                            </a>
                                        </footer>
                                    </div>
                                </div>
                            </section>
                            <section class="section" id="todo-list">
                                <div class=is_hidden>
                                    <div class="level">
                                        <div class="level-right">
                                            <div class="level-item">
                                                <strong>{ self.state.total() }</strong>
                                                { " item(s) left" }
                                            </div>
                                            <div class="level-item">
                                                <button class=("button", "is-danger")
                                                    onclick=self.link.callback(|_| Message::ClearCompleted)
                                                >
                                                    { format!("Clear completed ({})", self.state.total_completed()) }
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                    <p>{ "Double-click to edit a todo" }</p>
                                </div>
                                {
                                    for self.state.entries.iter()
                                        .filter(|entry| self.state.filter.fits(entry))
                                        .enumerate()
                                        .map(|entry| self.view_entry(entry))
                                }
                            </section>
                        </div>
                    </div>
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        <p>{ "Written by Joseph R. Quinn" }</p>
                        <p>{ "Adapted from " }<a href="http://todomvc.com" target="_blank">{ "TodoMVC" }</a></p>
                    </div>
                </footer>
            </>
        }
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Html {
        let selected = if self.state.filter == filter {
            "selected"
        } else {
            "not-selected"
        };

        html! {
            <a class=("card-footer-item", selected)
                href=filter.as_href()
                onclick=self.link.callback(move |_| Message::SetFilter(filter))
            >
                { filter }
            </a>
        }
    }

    fn view_input(&self) -> Html {
        html! {
            <input class="input"
                placeholder="What needs to be done?"
                value=&self.state.value
                oninput=self.link.callback(|event: InputData| Message::Update(event.value))
                onkeypress=self.link.batch_callback(|event: KeyboardEvent| {
                    if event.key() == "Enter" { Some(Message::Add) } else { None }
                })
            />
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut class = Classes::from("todo");

        if entry.editing {
            class.push(" editing");
        }

        if entry.completed {
            class.push(" completed");
        }

        let is_hidden = if entry.editing {
            "is-hidden"
        } else {
            ""
        };

        html! {
            <div class="box">
                <article class="media">
                    <div class="media-left">
                        <div class="control">
                            <input type="checkbox"
                                class="checkbox"
                                checked=entry.completed
                                onclick=self.link.callback(move |_| Message::Toggle(idx))
                            />
                        </div>
                    </div>
                    <div class="media-content">
                        <div class="content">
                            <span class=is_hidden
                                ondblclick=self.link.callback(move |_| Message::ToggleEdit(idx))>
                                { &entry.description }
                            </span>
                            { self.view_entry_edit_input((idx, &entry)) }
                        </div>
                    </div>
                    <div class="media-right">
                        <button class="delete"
                            onclick=self.link.callback(move |_| Message::Remove(idx))>
                        </button>
                    </div>
                </article>
            </div>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry)) -> Html {
        if entry.editing {
            html! {
                <input class="input"
                    type="text"
                    ref=self.focus_ref.clone()
                    value=&self.state.edit_value
                    onmouseover=self.link.callback(|_| Message::Focus)
                    oninput=self.link.callback(|event: InputData| Message::UpdateEdit(event.value))
                    onblur=self.link.callback(move |_| Message::Edit(idx))
                    onkeypress=self.link.batch_callback(move |event: KeyboardEvent| {
                        if event.key() == "Enter" { Some(Message::Edit(idx)) } else { None }
                    })
                />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}
