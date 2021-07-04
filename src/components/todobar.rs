use yew::ShouldRender;
use yew::Html;
use yew::html;
use yew::Component;
use yew::ComponentLink;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct TodoProps {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub index: usize,
}

pub struct TodoBar {
    link: ComponentLink<Self>,
    props: TodoProps,
}

impl Component for TodoBar {
    type Message = ();
    type Properties = TodoProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self { 
        TodoBar { link, props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender { 
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender  {
        if self.props != props {
            self.props = props;
            return true;
        } 
        return false;
    }

    fn view(&self) -> Html { 
        html!{
            <div class="todo-mainbar mainbar">
                <p class="todo-index edit-title todo-content">{format!("#{}",&self.props.index)}</p>
                <p class="todo-text todo-content">{&self.props.text}</p>
            </div>
        }
    }
}