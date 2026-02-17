use pelican_ui::{drawables, colors, Context, Callback};
use pelican_ui::drawable::Drawable;
use pelican_ui::canvas::{Align, RgbaImage, ShapeType, Image};
use pelican_ui::theme::Theme;
use pelican_ui::utils::{TitleSubtitle};
use pelican_ui::components::list_item::{ListItemSection, ListItemInfoLeft, ListItem as PelicanListItem};
use pelican_ui::components::{Checkbox, CheckboxList, TextInput, RadioSelector, Icon, DataItem, QRCode, NumericalInput};
use pelican_ui::components::text::{ExpandableText, TextStyle, TextSize};
use pelican_ui::components::avatar::{Avatar, AvatarSize, AvatarContent, AvatarIconStyle};
use pelican_ui::components::button::SecondaryButton;

use std::sync::Arc;

use crate::ChkBuilder;
use crate::flow::Flow;
use crate::closure::{EditedFn};

#[derive(Debug, Clone, PartialEq)]
pub enum Input {
    Text {label: String, actions: Option<Vec<Action>>, show_label: bool, preset: Option<String>},
    Currency {instructions: String}, //, on_edited: Box<dyn EditedFn>},
    Date {instructions: String}, //, on_edited: Box<dyn EditedFn>},
    Time {instructions: String}, //, on_edited: Box<dyn EditedFn>},
    Enumerator {items: Vec<EnumItem>},
    Avatar {content: AvatarContent, flair: Option<(String, AvatarIconStyle)>, action: Option<Action>},
    Boolean {items: Vec<ChecklistItem>}
}

impl Input {
    pub fn currency(instructions: &str) -> Self { //}, on_edited: impl FnMut(&mut Context, &mut String) + Clone + 'static) -> Self {
        Input::Currency {instructions: instructions.to_string()} //, on_edited: Box::new(on_edited)}
    }

    pub fn date(instructions: &str) -> Self { //, on_edited: impl FnMut(&mut Context, &mut String) + Clone + 'static) -> Self {
        Input::Date {instructions: instructions.to_string()} //, on_edited: Box::new(on_edited)}
    }

    pub fn time(instructions: &str) -> Self { //, on_edited: impl FnMut(&mut Context, &mut String) + Clone + 'static) -> Self {
        Input::Time {instructions: instructions.to_string()} //, on_edited: Box::new(on_edited)}
    }

    pub fn enumerator(items: Vec<EnumItem>) -> Self {
        Input::Enumerator {items}
    }

    pub fn text(label: &str, show_label: bool, preset: Option<String>, actions: Option<Vec<Action>>) -> Self {
        Input::Text {label: label.to_string(), show_label, preset, actions}
    }

    pub fn avatar(content: AvatarContent, flair: Option<(String, AvatarIconStyle)>, action: Option<Action>) -> Self {
        Input::Avatar {content, flair, action}
    }

    pub fn checklist(items: Vec<ChecklistItem>) -> Self {
        Input::Boolean {items}
    }

    pub fn build(&self, builder: &ChkBuilder) -> Option<Vec<Box<dyn Drawable>>> {
        let theme: &Theme = builder.theme();
        Some(match self {
            Input::Text {show_label, label, preset, ..} => drawables![TextInput::new(&theme, preset.as_deref(), show_label.then_some(label), Some(&format!("Enter {}...", label.to_lowercase())), None, None)],
            Input::Enumerator {items} => drawables![RadioSelector::new(&theme, 0, items.iter().map(|item| item.get()).collect::<Vec<_>>())],
            Input::Currency {instructions} => drawables![NumericalInput::numerical(&theme, instructions)],
            Input::Date {instructions} => drawables![NumericalInput::date(&theme, instructions)],
            Input::Time {instructions} => drawables![NumericalInput::time(&theme, instructions)],
            Input::Avatar {content, flair, action} => drawables![Avatar::new(&theme, content.clone(), flair.clone(), flair.is_some(), AvatarSize::Xxl, action.as_ref().map(|a| a.get()))],
            Input::Boolean {items} => drawables![CheckboxList::new(items.iter().map(|item| item.clone().get(&theme)).collect::<Vec<_>>())]
        })
    }
}

#[derive(Debug, Clone)]
pub enum Display {
    Text {text: String, size: TextSize, style: TextStyle, align: Align},
    Icon {icon: String},
    Image {image: Arc<RgbaImage>, size: (f32, f32)},
    Review {label: String, data: String, instructions: String},
    Table {label: String, items: Vec<TableItem>},
    Currency {amount: f32, instructions: String},
    List {label: Option<String>, items: Vec<ListItem>, instructions: Option<String>},
    QRCode {data: String, instructions: String},
    Avatar {content: AvatarContent},
    Actions {actions: Vec<ActionItem>}
}

impl Display {
    pub fn text(text: &str) -> Self {
        Display::Text {text: text.to_string(), size: TextSize::Md, style: TextStyle::Primary, align: Align::Left}
    }

    pub fn instructions(text: &str) -> Self {
        Display::Text {text: text.to_string(), size: TextSize::Md, style: TextStyle::Secondary, align: Align::Center}
    }

    pub fn label(text: &str) -> Self {
        Display::Text {text: text.to_string(), size: TextSize::H5, style: TextStyle::Heading, align: Align::Left}
    }

    pub fn icon(icon: &str) -> Self {
        Display::Icon {icon: icon.to_string()}
    }

    pub fn image(image: Arc<RgbaImage>, size: (f32, f32)) -> Self {
        Display::Image {image, size}
    }

    pub fn review(label: &str, data: &str, instructions: &str) -> Self {
        Display::Review {label: label.to_string(), data: data.to_string(), instructions: instructions.to_string()}
    }

    pub fn table(label: &str, items: Vec<TableItem>) -> Self {
        Display::Table {label: label.to_string(), items}
    }

    pub fn qr_code(data: &str, instructions: &str) -> Self {
        Display::QRCode {data: data.to_string(), instructions: instructions.to_string()}
    }

    pub fn list(label: Option<&str>, mut items: Vec<ListItem>, instructions: Option<&str>, flow: Option<Flow>) -> Self {
        items.iter_mut().for_each(|item| {item.flow = flow.clone()});
        Display::List{label: label.map(|i| i.to_string()), items, instructions: instructions.map(|i| i.to_string())}
    }

    pub fn currency(amount: f32, instructions: &str) -> Self {
        Display::Currency {amount, instructions: instructions.to_string()}
    }

    pub fn avatar(content: AvatarContent) -> Self {
        Display::Avatar {content}
    }

    pub fn actions(actions: Vec<ActionItem>) -> Self {
        Display::Actions {actions}
    }

    pub fn build(&mut self, builder: &ChkBuilder) -> Option<Vec<Box<dyn Drawable>>> {
        let theme: &Theme = builder.theme();
        Some(match self {
            Display::Icon {icon} => drawables![Icon::new(&theme, icon, Some(theme.colors().get(colors::Text::Heading)), 128.0)],
            Display::Image {image, size} => drawables![Image{shape: ShapeType::Rectangle(0.0, *size, 0.0), image: image.clone(), color: None}],
            Display::Text {text, size, style, align} if !text.is_empty() => drawables![ExpandableText::new(&theme, text, *size, *style, *align, None)],
            Display::Review {label, data, instructions} => drawables![DataItem::text(&theme, label, data, instructions, Some(Vec::<(String, Option<String>, Box<dyn Callback>)>::new()))],
            Display::Table {label, items} => drawables![DataItem::table(&theme, label, items.iter().map(|TableItem{title, data}| (title.clone(), data.clone())).collect(), Some(Vec::<(String, Option<String>, Box<dyn Callback>)>::new()))],
            Display::Currency {amount, instructions} => drawables![NumericalInput::display(&theme, instructions)],
            Display::List {items, instructions, ..} if items.is_empty() => drawables![ExpandableText::new(&theme, instructions.as_ref()?, TextSize::Md, TextStyle::Secondary, Align::Center, None)],
            Display::List {label, items, ..} => drawables![ListItemSection::new(&theme, label.clone(), items.into_iter().map(|item| item.build(&builder)).collect::<Vec<_>>())],
            Display::QRCode {data, instructions} => drawables![QRCode::new(&theme, data), ExpandableText::new(&theme, instructions, TextSize::Md, TextStyle::Secondary, Align::Center, None)],
            Display::Avatar {content} => drawables![Avatar::new(&theme, content.clone(), None, false, AvatarSize::Xxl, None)],
            Display::Actions {actions} => actions.into_iter().map(|ActionItem(a, l, i)| Box::new(SecondaryButton::medium(&theme, &i, &l, None, a.get())) as Box<dyn Drawable>).collect::<Vec<_>>(),
            _ => return None
        })
    }
}

#[derive(Debug, Clone)]
pub struct ListItem {
    avatar: Option<AvatarContent>, 
    title: String, 
    subtitle: String, 
    secondary: Option<String>, 
    flow: Option<Flow>, 
    on_click: Box<dyn Callback>
}

impl ListItem {
    pub fn plain(title: &str, subtitle: &str, secondary: Option<&str>, on_click: impl FnMut(&mut Context, &Theme) + 'static + Clone) -> Self {
        ListItem {
            avatar: None,
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            secondary: secondary.map(|s| s.to_string()),
            flow: None,
            on_click: Box::new(on_click)
        }
    }

    pub fn avatar(avatar: AvatarContent, title: &str, subtitle: &str, secondary: Option<&str>, on_click: impl FnMut(&mut Context, &Theme) + 'static + Clone) -> Self {
        ListItem {
            avatar: Some(avatar),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            secondary: secondary.map(|s| s.to_string()),
            flow: None,
            on_click: Box::new(on_click)
        }
    }

    pub(crate) fn build(&self, builder: &ChkBuilder) -> PelicanListItem {
        let ListItem {avatar, title, subtitle, secondary, flow, on_click} = self.clone();
        let has_flow = flow.is_some();
        let closure = Box::new(move |ctx: &mut Context, theme: &Theme| {
            (on_click.clone())(ctx, theme);
            if let Some(mut f) = flow.clone() {(f.build(ctx))(ctx, theme);}
        });

        let theme: &Theme = builder.theme();
        PelicanListItem::new(&theme, avatar.clone(), 
            ListItemInfoLeft::new(&title, Some(&subtitle), None, None), 
            secondary.as_ref().map(|s| TitleSubtitle::new(s, Some("Details"))), 
            None, has_flow.then_some("forward"), closure
        )
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Share {data: String},
    SelectImage,
    Custom {action: Box<dyn Callback>},
    None,
    Flow {flow: Flow},
    // Navigate {flow: Flow},
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Action::Share { data: a }, Action::Share { data: b }) => a == b,
            (Action::Flow {..}, Action::Flow {..}) => true,
            (Action::SelectImage, Action::SelectImage) => true,
            (Action::None, Action::None) => true,
            _ => false,
        }
    }
}

impl Action {
    pub fn share(data: &str) -> Self {
        Action::Share {data: data.to_string()}
    }

    pub fn select_image() -> Self {
        Action::SelectImage
    }

    pub fn custom(action: impl Callback + 'static) -> Self {
        Action::Custom {action: Box::new(action)}
    }

    pub fn flow(flow: Flow) -> Self {
        Action::Flow {flow}
    }

    // pub fn navigate(flow: Flow) -> Self {
    //     Action::Navigate {flow}
    // }

    pub fn get(&self) -> Box<dyn Callback> {
        match self {
            Action::Share {data} => {
                let share_data = data.clone();
                Box::new(move |_ctx: &mut Context, _: &Theme| println!("Sharing data {:?}", share_data.clone()))
            }

            Action::SelectImage => {
                Box::new(move |_ctx: &mut Context, _: &Theme| println!("Selecting image"))
            }

            Action::Custom {action} => {
                let mut action = action.clone();
                Box::new(move |ctx: &mut Context, theme: &Theme| (action)(ctx, theme))
            }

            Action::Flow {flow} => {
                let mut flow = flow.clone();
                Box::new(move |ctx: &mut Context, _: &Theme| {flow.build(ctx);})
            }

            // Action::Navigate {flow} => flow.clone().build(),

            _ => Box::new(move |_ctx: &mut Context, _: &Theme| println!("Doing nothing here..."))
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ActionItem(Action, String, String);
impl ActionItem {
    pub fn new(action: Action, label: &str, icon: &str) -> Self {
        ActionItem(action, label.to_string(), icon.to_string())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TableItem {title: String, data: String}

impl TableItem {
    pub fn new(title: &str, data: &str) -> Self {
        TableItem { title: title.to_string(), data: data.to_string() }
    }
}

#[derive(Clone)]
pub struct EnumItem {title: String, data: String} //, callback: Box<dyn Callback>}
impl std::fmt::Debug for EnumItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumItem").field("title", &self.title).field("data", &self.data).finish()
    }
}

impl EnumItem {
    pub fn new(title: &str, data: &str) -> Self { //, callback: impl FnMut(&mut Context) + Clone + 'static) -> Self {
        EnumItem {title: title.to_string(), data: data.to_string(), } //callback: Box::new(callback)}
    }

    fn get(&self) -> (&str, &str, Box<dyn Callback>) {
        // let mut callback = self.callback.clone();
        (&self.title as &str, &self.data as &str, Box::new(|_, _|{})) //Box::new(move |ctx: &mut Context| {(callback)(ctx)}) as Box<dyn FnMut(&mut Context)>)
    }
}

impl PartialEq for EnumItem {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title &&
        self.data == other.data
    }
}

#[derive(Debug, Clone)]
pub struct ChecklistItem {title: String, subtitle: Option<String>, is_selected: bool, on_check: Box<dyn Callback>, on_uncheck: Box<dyn Callback>}
impl ChecklistItem {
    pub fn new(title: &str, subtitle: Option<&str>, is_selected: bool, on_check: Box<dyn Callback>, on_uncheck: Box<dyn Callback>) -> Self {
        ChecklistItem {title: title.to_string(), subtitle: subtitle.map(|s| s.to_string()), is_selected, on_check, on_uncheck}
    }

    fn get(self, theme: &Theme) -> Checkbox {
        Checkbox::new(theme, &self.title, self.subtitle, self.is_selected, self.on_check, self.on_uncheck)
    }
}

impl PartialEq for ChecklistItem {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title &&
        self.subtitle == other.subtitle &&
        self.is_selected == other.is_selected
    }
}