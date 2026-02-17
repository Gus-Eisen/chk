use chk::{
    RootInfo, FormItem, NumberVariant, SuccessClosure, Flow, Bumper, ActionItem,
    Display, Offset, Context, Screen, PageType, PageBuilder,
    Color, Theme, ChkBuilder, Form, Root
};
use chk::items::{ListItem, Action, EnumItem, TableItem};

pub struct _TaskManager;

impl chk::App for _TaskManager {
    fn roots(&self, ctx: &mut Context, builder: &ChkBuilder) -> Vec<RootInfo> {
        // ctx.state.insert(Tasks(vec![
        //     Task {
        //         title: "Buy groceries".into(),
        //         description: "Milk, eggs".into(),
        //         due_date: "01/25/26".into(),
        //         priority: Priority::Medium,
        //         completed: false,
        //     },
        //     Task {
        //         title: "Finish report".into(),
        //         description: "Q4 numbers".into(),
        //         due_date: "01/30/26".into(),
        //         priority: Priority::High,
        //         completed: false,
        //     },
        // ]));

        let home = TaskList;
        vec![RootInfo::icon("check", "Tasks", TaskList::new(builder).build(ctx, builder))]
    }

    fn theme(&self) -> Theme {Theme::Dark(Color::from_hex("#4567EE", 255))}
}

#[derive(Debug, Clone)]
pub struct TaskList;

impl TaskList {
    fn new(builder: &ChkBuilder) -> PageType {
        let items = Tasks::test().0.iter().map(|t| {
            let task = t.clone();
            ListItem::plain(&t.title, &format!("Due {}", t.due_date), Some(&t.priority.to_string()), move |ctx, theme| {})
        }).collect();

        let form = NewTaskForm::new(builder).0;

        Root::new("Tasks",
            vec![Display::list(None, items, None, Some(Flow::new(vec![Screen::new_builder(builder, TaskDetails::new())])))], //Some(Flow::new(vec![Screen::new_builder(builder, TaskDetails)])))],
            None, ("New Task".into(), Flow::from_form(form)), None,
        )
    }

    // TODO: Wrap Root in a listener instead.
    // let tasks = ctx.state.get::<Tasks>().unwrap().0.clone();
    // (self.0 != tasks).then(|| self.0 = tasks).is_some()
}

#[derive(Debug, Clone)]
pub struct NewTaskForm(Form);

impl NewTaskForm {
    pub fn new(builder: &ChkBuilder) -> Self {
        let closure = Box::new(|ctx: &mut Context| {
            // let task = ctx.state.get::<NewTask>().unwrap().0.clone();
            // ctx.state.get_mut_or_default::<Tasks>().0.push(task);
        });
        
        NewTaskForm(Form::new(builder, vec![
            FormItem::text("Title"),
            FormItem::text("Description"),
            FormItem::number("Due date", NumberVariant::Date),
            FormItem::enumerator("Priority", vec![
                ("Low", "Within 1-2 weeks"),
                ("Medium", "Within 7 days"),
                ("High", "Within 1-2 days"),
            ]),
        ], closure))
    }
}

// impl chk::Form for NewTaskForm {
//     fn inputs(&self) -> Vec<FormItem> {
//         vec![
//             FormItem::text("Title", |c| &mut c.state.get_mut_or_default::<NewTask>().0.title),
//             FormItem::text("Description", |c| &mut c.state.get_mut_or_default::<NewTask>().0.description),
//             FormItem::number("Due date", NumberVariant::Date, |c| &mut c.state.get_mut_or_default::<NewTask>().0.due_date),
//             FormItem::enumerator("Priority", vec![
//                 EnumItem::new("Low", "Within 1-2 weeks", |c| c.state.get_mut_or_default::<NewTask>().0.priority = Priority::Low),
//                 EnumItem::new("Medium", "Within 7 days", |c| c.state.get_mut_or_default::<NewTask>().0.priority = Priority::Medium),
//                 EnumItem::new("High", "Within 1-2 days", |c| c.state.get_mut_or_default::<NewTask>().0.priority = Priority::High),
//             ]),
//         ]
//     }

//     fn success(&self) -> Box<dyn SuccessClosure> {
//         Box::new(|ctx| {
//             let t = &ctx.state.get::<NewTask>().unwrap().0;
//             ["Task created".into(), "check".into(), format!("Added {}", t.title)]
//         })
//     }

//     fn on_submit(&self, ctx: &mut Context) {
//         let task = ctx.state.get::<NewTask>().unwrap().0.clone();
//         ctx.state.get_mut_or_default::<Tasks>().0.push(task);
//     }
// }

#[derive(Debug, Clone)]
pub struct TaskDetails;


impl TaskDetails {
    fn new() -> Box<dyn PageBuilder> {
        Box::new(|builder: &ChkBuilder| {
            let task = Task::new("Test Task"); //ctx.state.get::<Task>().unwrap();
            PageType::display(
                &task.title,
                vec![
                    Display::table(
                        "Task details",
                        vec![
                            TableItem::new("Title", &task.title),
                            TableItem::new("Due date", &task.due_date),
                            TableItem::new("Priority", &task.priority.to_string()),
                            TableItem::new("Completed", if task.completed { "Yes" } else { "No" }),
                        ],
                    ),
                    Display::text(&format!("Description\n\n{}", task.description)),
                    // Display::actions(vec![ActionItem::new(Action::custom(|c| {
                    //     c.state.get_mut::<Task>().unwrap().completed = true
                    // }), "Mark completed", "checkmark")]),
                ],
                None,
                Bumper::Done,
                Offset::Start,
            )
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct NewTask(Task);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tasks(Vec<Task>);

impl Tasks {
    pub fn test() -> Self {
        Tasks(vec![
            Task::new("Wash dog"),
            Task::new("Clean out car"),
            Task::new("Do dishes"),
        ])
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Task {
    title: String,
    description: String,
    due_date: String,
    priority: Priority,
    completed: bool,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Task {
            title: title.to_string(),
            description: title.to_string(),
            due_date: "Someday".to_string(),
            priority: Priority::Low,
            completed: false,
        }
    }
}
