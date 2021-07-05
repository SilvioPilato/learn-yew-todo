use yew::ShouldRender;
use yew::Html;
use yew::html;
use yew::Component;
use yew::ComponentLink;
use yew::Properties;
use yew::Callback;
use yew_octicons::Icon;
use yew_octicons::IconKind;

#[derive(Properties, Clone, PartialEq)]
pub struct TodoProps {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub done: bool,
    #[prop_or_default]
    pub index: usize,
    #[prop_or_default]
    pub on_delete: Callback<usize>,
    #[prop_or_default]
    pub on_done: Callback<usize>,
}

pub enum Msg {
    Done,
    Remove,
}

pub struct TodoBar {
    link: ComponentLink<Self>,
    props: TodoProps,
}

impl Component for TodoBar {
    type Message = Msg;
    type Properties = TodoProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self { 
        TodoBar { link, props }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender { 
        match message {
            Msg::Remove => {
                self.props.on_delete.emit(self.props.index);
                return false;
            },
            Msg::Done => {
                self.props.on_done.emit(self.props.index);
                return false;
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender  {
        if self.props != props {
            self.props = props;
            return true;
        } 
        return false;
    }

    fn view(&self) -> Html {
        let textClass = (self.props.done).then(||"text-disabled").unwrap_or("");
        html!{
            <div class="todo-mainbar mainbar">
                <p class="todo-index edit-title todo-content todo-side">{format!("#{}",&self.props.index)}</p>
                <div class="todo-text-div">
                    <p class=format!("todo-text todo-content {}", textClass)>{&self.props.text}</p>
                </div>
                <span onclick=self.link.callback(|_| Msg::Done) class="todo-side todo-conficon grow">
                    { Icon::new(IconKind::CheckCircleFill) }
                </span>
                <span onclick=self.link.callback(|_| Msg::Remove) class="todo-side todo-delicon grow">
                    { Icon::new(IconKind::XCircleFill) }
                </span>
            </div>
        }
    }
}