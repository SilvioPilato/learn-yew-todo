use yew::Callback;
use yew::InputData;
use yew::ShouldRender;
use yew::Html;
use yew::html;
use yew::Component;
use yew::ComponentLink;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_new: Callback<String>,
}

pub struct EditBar {
    link: ComponentLink<Self>,
    current_text: String,
    props: Props
}

pub enum Msg {
    CreateNew,
    TextChanged(String),
}

impl Component for EditBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self { 
        EditBar { current_text: String::new(), link, props }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender { 
        match message {
            Msg::CreateNew => {
                self.props.on_new.emit(String::from(&self.current_text));
                false
            },
            Msg::TextChanged(text) => {
                self.current_text = text;
                false
            }
        } 
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender  { 
        false
    }

    fn view(&self) -> Html {
        html!{
            <div class="edit-mainbar mainbar">
                <h2 class="edit-title">{"Create a new TODO element"}</h2>
                <textarea class="edit-textbar" oninput=self.link.callback(|data: InputData | Msg::TextChanged(data.value)) type="text" id="editbar-text" name="editbar-text"/>
                <button class="edit-createbutton" onclick=self.link.callback(|_| Msg::CreateNew)>{ "Create" }</button>
            </div>
        }
    }
}