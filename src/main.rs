mod components;
use components::title::Title;
use components::editbar::EditBar;
use components::todobar::TodoBar;
use yew::prelude::*;

enum Msg {
    CreateNew(String),
    RemoveOne(usize),
    DoneOne(usize),
}

pub struct Todo {
    text: String,
    done: bool,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    todos: Vec<Todo>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: Vec::new()
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::CreateNew(data) => {
                match data.is_empty() {
                    true => {
                        return false;
                    }
                    false => {
                        self.todos.push(Todo { text: data, done: false});
                        return true;
                    }
                }
            },
            Msg::RemoveOne(index) => {
                self.todos.remove(index);
                return true;
            },
            Msg::DoneOne(index) => {
                self.todos.get_mut(index).unwrap().done = true;
                return true;
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let cb = &self.link.callback(Msg::CreateNew);
        html! {
            <div class="main">
                <Title text={"A Rusty todo app"} />
                <EditBar on_new=cb />
                {for self.todos.iter().enumerate().map(|(index, todo)| get_todo_component(index, &todo, &self.link))}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

fn get_todo_component(index: usize, todo: &Todo, link: &ComponentLink<Model>) -> Html {
    let del_cb = link.callback(Msg::RemoveOne);
    let done_cb = link.callback(Msg::DoneOne);
    html! {
        <TodoBar text={String::from(&todo.text)} done={todo.done} index={index} on_delete=del_cb on_done=done_cb/>
    }
}