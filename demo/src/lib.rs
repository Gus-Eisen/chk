use chk::{FormItem, SuccessClosure, Success, Flow, Bumper, RootP, Display, RootBuilder, layout::Offset, Context, drawable::{Drawable, Component}, Screen, components::Circle, event::{OnEvent, Event}, layout::Stack, PageType, PageBuilder, ScreenBuilder};
use chk::components::interface::{Interface, RootInfo};
use ramp::prism;

use chk::items::{ListItem, Action, Input, EnumItem, TableItem};

use std::sync::Arc;
use image::RgbaImage;

pub struct _AddressBook;
impl chk::App for _AddressBook {
    fn roots(&self, ctx: &mut Context) -> Vec<RootInfo> {
        ctx.state.insert(People(vec![
            Person {name: "Annie".to_string(), phone_number: "406-802-2162".to_string(), address: "Spiegel 2".to_string(), info: String::new(), status: Status::Engaged},
            Person {name: "Dave".to_string(), phone_number:  "406-802-2162".to_string(), address: "Spiegel 2".to_string(), info: String::new(), status: Status::Married},
            Person {name: "Danny".to_string(), phone_number:  "406-802-2162".to_string(), address: "Spiegel 2".to_string(), info: String::new(), status: Status::Single},
        ]));

        let home = AddressBook::new(ctx);
        vec![RootInfo::icon("book", "Book", Box::new(Screen::new(ctx, home)))]
    }
}

chk::run!{|ctx: &mut Context| _AddressBook}

#[derive(Debug, Clone)]
pub struct AddressBook(Vec<Person>);
impl AddressBook {
    pub fn new(ctx: &mut Context) -> Self {
        AddressBook(ctx.state.get::<People>().unwrap().0.clone())
    }
}

impl chk::Root for AddressBook {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn RootBuilder> {
        Box::new(move |ctx: &mut Context| {
            let people = ctx.state.get::<People>().unwrap().0.iter().map(|p| {
                let person = p.clone();
                ListItem::plain(&p.name, &p.address, Some(&p.phone_number), move |ctx: &mut Context| {ctx.state.insert(person.clone());})
            }).collect::<Vec<ListItem>>();

            RootP::new("Address book", 
                vec![Display::list(None, people, None, Some(Flow::new(vec![Screen::new_builder(ContactDetails)])))], 
                None, ("New contact".to_string(), Flow::from_form(NewContactForm)), None, 
            )
        })
    }

    fn redraw(&mut self, ctx: &mut Context) -> bool {
        let people = ctx.state.get::<People>().unwrap().0.clone();
        (self.0 != people).then(|| self.0 = people).is_some()
    }
}

#[derive(Debug, Clone)]
pub struct NewContactForm;
impl chk::Form for NewContactForm {
    fn inputs(&self) -> Vec<FormItem> {vec![
        FormItem::text("Contact name", |ctx: &mut Context| &mut ctx.state.get_mut_or_default::<NewContact>().0.name),
        FormItem::text("Phone number", |ctx: &mut Context| &mut ctx.state.get_mut_or_default::<NewContact>().0.phone_number),
        FormItem::text("Address", |ctx: &mut Context| &mut ctx.state.get_mut_or_default::<NewContact>().0.address),
        FormItem::text("Additional info", |ctx: &mut Context| &mut ctx.state.get_mut_or_default::<NewContact>().0.info),
        FormItem::enumerator("Marital status", vec![
            EnumItem::new("Married", "No, you cannot have this person", |ctx: &mut Context| ctx.state.get_mut_or_default::<NewContact>().0.status = Status::Married),
            EnumItem::new("Engaged", "You really should not try to take this person", |ctx: &mut Context| ctx.state.get_mut_or_default::<NewContact>().0.status = Status::Married),
            EnumItem::new("Dating", "Kinda rude, but go for it buddy", |ctx: &mut Context| ctx.state.get_mut_or_default::<NewContact>().0.status = Status::Dating),
            EnumItem::new("Single", "Have at 'em!", |ctx: &mut Context| ctx.state.get_mut_or_default::<NewContact>().0.status = Status::Single),
        ])
    ]}

    fn success(&self) -> Box<dyn SuccessClosure> {Box::new(|ctx: &mut Context| {
        let person = &ctx.state.get_mut_or_default::<NewContact>().0;
        ["Contact created".to_string(), "profile".to_string(), format!("Added {} to your contacts", person.name)]
    })}

    fn on_submit(&self, ctx: &mut Context) {
        let contact = ctx.state.get::<NewContact>().unwrap().0.clone();
        ctx.state.get_mut_or_default::<People>().0.push(contact);
    }
}

#[derive(Debug, Clone)]
pub struct ContactDetails;
impl chk::Page for ContactDetails {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(move |ctx: &mut Context| {
            let person = &ctx.state.get::<Person>().unwrap();
            PageType::display(&person.name, vec![
                Display::table("Contact's details", vec![
                    TableItem::new("Full Name", &person.name),
                    TableItem::new("Phone Number", &person.phone_number),
                    TableItem::new("Address", &person.address),
                    TableItem::new("Maritial status", &person.status.to_string()),
                ]),
                Display::text(&format!("Additional information\n\n{}", &person.info)),
            ], None, Bumper::Done, Offset::Start)
        })
    }
}


#[derive(Debug, Default, Clone, PartialEq)]
pub enum Status {
    Married,
    Engaged,
    Dating,
    #[default]
    Single
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Status::Married => "Married",
            Status::Engaged => "Engaged",
            Status::Dating => "Dating",
            Status::Single => "Single",
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct NewContact(Person);
#[derive(Debug, Default, Clone, PartialEq)]
pub struct People(Vec<Person>);
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Person {
    name: String,
    phone_number: String,
    address: String,
    info: String,
    status: Status
}