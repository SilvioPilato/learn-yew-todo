use yew::ShouldRender;
use yew::Html;
use yew::html;
use yew::Component;
use yew::ComponentLink;
use yew::Properties;
use yew_octicons::Icon;
use yew_octicons::IconKind;

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
                <p class="todo-index edit-title todo-content todo-side">{format!("#{}",&self.props.index)}</p>
                <div class="todo-text-div">
                    <p class="todo-text todo-content">{&self.props.text}</p>
                </div>
                <div class="todo-side todo-delicon">
                    { Icon::new(IconKind::Trashcan) }
                </div>
            </div>
        }
    }
}