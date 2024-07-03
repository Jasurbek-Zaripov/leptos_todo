use leptos::*;
use leptos::ev::{Event, MouseEvent};

use crate::pages::app::Status;
use crate::pages::app::TodoItemStruct;

pub struct TodoItemProp {
    pub(crate) value: TodoItemStruct,
    pub(crate) on_text: Box<dyn Fn(TodoItemStruct)>,
    pub(crate) on_complete: Box<dyn Fn(TodoItemStruct)>,
    pub(crate) on_time: Box<dyn Fn(TodoItemStruct)>,
    pub(crate) on_remove: Box<dyn Fn(usize)>,
}

#[component]
pub fn TodoItem(prop: TodoItemProp) -> impl IntoView {
    let (edit, set_edit) = create_signal(false);
    let (todo, set_todo) = create_signal(prop.value.text.clone());
    let (status, set_status) = create_signal(prop.value.status.clone());
    let (time, set_time) = create_signal(prop.value.created_at.clone());

    let on_change_text = move |event: Event| {
        set_todo.set(event_target_value(&event));
        if time.get().len() > 0 && todo.get().len() > 0 {
            (prop.on_text)(TodoItemStruct {
                id: prop.value.id.clone(),
                text: todo.get(),
                status: status.get(),
                created_at: time.get(),
            });
            set_edit.set(false);
        }
    };

    let on_click_status = move |event: Event| {
        set_status.set(event_target_value(&event));
        (prop.on_complete)(TodoItemStruct {
            id: prop.value.id.clone(),
            text: todo.get(),
            status: status.get(),
            created_at: time.get(),
        });

        set_edit.set(false);
    };

    let on_click_remove = move |_: MouseEvent| {
        (prop.on_remove)(prop.value.id.clone());
    };

    let on_change_time = move |event: Event| {
        set_time.set(event_target_value(&event));
        if time.get().len() > 0 && todo.get().len() > 0 {
            (prop.on_time)(TodoItemStruct {
                id: prop.value.id.clone(),
                text: todo.get(),
                status: status.get(),
                created_at: time.get(),
            });
            set_edit.set(false);
        }
    };

    let is_completed = move || status.get() == Status::Completed.value();

    view! {
        <div>

                <input
                    type="text"
                    readonly=move || !edit.get()
                    disabled=move || is_completed()
                    on:dblclick=move |_| if !is_completed() { set_edit.set(true)}
                    prop:value=move || todo.get()
                    on:change={on_change_text}
                />


                <input
                    type="datetime-local"
                    prop:value=move || time.get()
                    on:change={on_change_time}
                    readonly=move || !edit.get()
                    disabled=move || is_completed()
                    on:dblclick=move |_| if !is_completed() { set_edit.set(true)}
                />

                <select prop:value=move || status.get() on:change={on_click_status}>
                    <option selected={move || status.get() == Status::Todo.value()} value={Status::Todo.value()}>{Status::Todo.value()}</option>
                    <option selected={move || status.get() == Status::Pending.value()} value={Status::Pending.value()}>{Status::Pending.value()}</option>
                    <option selected={move || status.get() == Status::Completed.value()} value={Status::Completed.value()}>{Status::Completed.value()}</option>
                </select>
            
                <button role="button" class="delete-btn" on:click={on_click_remove}>
                    <svg width="100%" fill="red" height="100%" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 110.61 122.88">
                        <path d="M39.27,58.64a4.74,4.74,0,1,1,9.47,0V93.72a4.74,4.74,0,1,1-9.47,0V58.64Zm63.6-19.86L98,103a22.29,22.29,0,0,1-6.33,14.1,19.41,19.41,0,0,1-13.88,5.78h-45a19.4,19.4,0,0,1-13.86-5.78l0,0A22.31,22.31,0,0,1,12.59,103L7.74,38.78H0V25c0-3.32,1.63-4.58,4.84-4.58H27.58V10.79A10.82,10.82,0,0,1,38.37,0H72.24A10.82,10.82,0,0,1,83,10.79v9.62h23.35a6.19,6.19,0,0,1,1,.06A3.86,3.86,0,0,1,110.59,24c0,.2,0,.38,0,.57V38.78Zm-9.5.17H17.24L22,102.3a12.82,12.82,0,0,0,3.57,8.1l0,0a10,10,0,0,0,7.19,3h45a10.06,10.06,0,0,0,7.19-3,12.8,12.8,0,0,0,3.59-8.1L93.37,39ZM71,20.41V12.05H39.64v8.36ZM61.87,58.64a4.74,4.74,0,1,1,9.47,0V93.72a4.74,4.74,0,1,1-9.47,0V58.64Z"/>
                    </svg>
                </button>

        </div>
    }
}