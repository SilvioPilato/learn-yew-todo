mod components;
use components::title::Title;
use components::editbar::EditBar;
use components::todobar::TodoBar;
use yew::prelude::*;

enum Msg {
    CreateNew(String)
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    todos: Vec<String>,
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
                        self.todos.push(data);
                        return true;
                    }
                }
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
                {for self.todos.iter().enumerate().map(|(index, text)| get_todo_component(index, String::from(text)))}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

fn get_todo_component(index: usize, text: String) -> Html {
    html! {
        <TodoBar text={text} index={index}/>
    }
}