use chk::{
    RootInfo, FormItem, NumberVariant, SuccessClosure, Flow, Bumper, ActionItem,
    RootP, Display, RootBuilder, Offset, Context, Screen, PageType, PageBuilder,
    Color, Theme, ChkBuilder
};
use chk::items::{ListItem, Action, EnumItem, TableItem};

/* ============================================================
   APP
============================================================ */

pub struct _Netflix;

impl chk::App for _Netflix {
    fn roots(&self, ctx: &mut Context, builder: ChkBuilder) -> Vec<RootInfo> {
        // Seed titles
        ctx.state.insert(Titles(vec![
            Title {
                id: 1,
                name: "The Byte Knight".into(),
                description: "A rogue compiler engineer fights bugs in a cyber-medieval world.".into(),
                year: 2026.to_string(),
                maturity: Maturity::TVMA,
                genre: Genre::Action,
                category: RowCategory::TrendingNow,
                duration_min: 48.to_string(),
                match_score: 97.to_string(),
                is_movie: false,
            },
            Title {
                id: 2,
                name: "Rust & Relax".into(),
                description: "A calming docuseries about memory safety, ownership, and cozy refactors.".into(),
                year: 2025.to_string(),
                maturity: Maturity::TV14,
                genre: Genre::Documentary,
                category: RowCategory::BecauseYouWatched,
                duration_min: 32.to_string(),
                match_score: 92.to_string(),
                is_movie: false,
            },
            Title {
                id: 3,
                name: "404: Love Not Found".into(),
                description: "A romantic comedy where two devs meet through a production outage.".into(),
                year: 2024.to_string(),
                maturity: Maturity::PG13,
                genre: Genre::Romance,
                category: RowCategory::NewReleases,
                duration_min: 106.to_string(),
                match_score: 88.to_string(),
                is_movie: true,
            },
            Title {
                id: 4,
                name: "Kitchen Nightmares: AI Edition".into(),
                description: "A chef and an LLM try to rescue restaurants from chaos.".into(),
                year: 2026.to_string(),
                maturity: Maturity::TVPG,
                genre: Genre::Reality,
                category: RowCategory::Top10,
                duration_min: 44.to_string(),
                match_score: 85.to_string(),
                is_movie: false,
            },
            Title {
                id: 5,
                name: "Planet: Garbage Collector".into(),
                description: "A sci-fi thriller about cleaning space debris before it cleans you.".into(),
                year: 2023.to_string(),
                maturity: Maturity::R,
                genre: Genre::SciFi,
                category: RowCategory::TrendingNow,
                duration_min: 121.to_string(),
                match_score: 90.to_string(),
                is_movie: true,
            },
        ]));

        // Seed user profile / watchlist / progress
        ctx.state.insert(UserProfile {
            name: "Guest".into(),
            plan: Plan::Standard,
        });

        ctx.state.insert(MyList(vec![1, 3])); // ids
        ctx.state.insert(ContinueWatching(vec![
            WatchingProgress { title_id: 1, progress_pct: 32 },
            WatchingProgress { title_id: 5, progress_pct: 74 },
        ]));

        let home = BrowseHome::new(ctx);
        let search = SearchRoot::new(ctx);
        let list = MyListRoot::new(ctx);

        let builder = ChkBuilder::new(Theme::Dark(Color::from_hex("#E50914", 255)));

        vec![
            RootInfo::icon("play", "Browse", Box::new(Screen::new(builder, home))),
            RootInfo::icon("search", "Search", Box::new(Screen::new(builder, search))),
            RootInfo::icon("bookmark", "My List", Box::new(Screen::new(builder, list))),
        ]
    }

    fn theme(&self) -> Theme {
        Theme::Dark(Color::from_hex("#E50914", 255))
    }
}

/* ============================================================
   DATA MODELS
============================================================ */

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Titles(pub Vec<Title>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Title {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub year: String,
    pub maturity: Maturity,
    pub genre: Genre,
    pub category: RowCategory,
    pub duration_min: String,
    pub match_score: String,
    pub is_movie: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MyList(pub Vec<u32>); // title ids

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContinueWatching(pub Vec<WatchingProgress>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WatchingProgress {
    pub title_id: u32,
    pub progress_pct: u8,
}

#[derive(Debug, Clone, Default)]
pub struct SelectedTitle(pub Title);

#[derive(Debug, Clone, Default)]
pub struct UserProfile {
    pub name: String,
    pub plan: Plan,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Plan {
    #[default]
    Basic,
    Standard,
    Premium,
}

impl std::fmt::Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Plan::Basic => "Basic",
            Plan::Standard => "Standard",
            Plan::Premium => "Premium",
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Genre {
    #[default]
    Action,
    Comedy,
    Drama,
    Documentary,
    Horror,
    Romance,
    Reality,
    SciFi,
    Kids,
}

impl std::fmt::Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Genre::Action => "Action",
            Genre::Comedy => "Comedy",
            Genre::Drama => "Drama",
            Genre::Documentary => "Documentary",
            Genre::Horror => "Horror",
            Genre::Romance => "Romance",
            Genre::Reality => "Reality",
            Genre::SciFi => "Sci-Fi",
            Genre::Kids => "Kids",
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Maturity {
    G,
    PG,
    #[default]
    PG13,
    R,
    TVPG,
    TV14,
    TVMA,
}

impl std::fmt::Display for Maturity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Maturity::G => "G",
            Maturity::PG => "PG",
            Maturity::PG13 => "PG-13",
            Maturity::R => "R",
            Maturity::TVPG => "TV-PG",
            Maturity::TV14 => "TV-14",
            Maturity::TVMA => "TV-MA",
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum RowCategory {
    #[default]
    TrendingNow,
    Top10,
    NewReleases,
    BecauseYouWatched,
}

impl std::fmt::Display for RowCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RowCategory::TrendingNow => "Trending Now",
            RowCategory::Top10 => "Top 10",
            RowCategory::NewReleases => "New Releases",
            RowCategory::BecauseYouWatched => "Because You Watched",
        })
    }
}

/* ============================================================
   ROOT: BROWSE HOME
============================================================ */

#[derive(Debug, Clone)]
pub struct BrowseHome {
    cached: Vec<Title>,
}

impl BrowseHome {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            cached: ctx.state.get::<Titles>().unwrap().0.clone(),
        }
    }
}

impl chk::Root for BrowseHome {
    fn page(&mut self, _: &mut Context) -> Box<dyn RootBuilder> {
        Box::new(|ctx| {
            let titles = ctx.state.get::<Titles>().unwrap().0.clone();

            // "Rows" simulated by lists grouped by category
            let mut trending = vec![];
            let mut top10 = vec![];
            let mut new_releases = vec![];
            let mut because = vec![];

            for t in titles.iter() {
                let item = {
                    let selected = t.clone();
                    ListItem::plain(
                        &t.name,
                        &format!("{} • {} • {}% match", t.year, t.maturity, t.match_score),
                        Some(&t.genre.to_string()),
                        move |ctx| {
                            ctx.state.insert(SelectedTitle(selected.clone()));
                        },
                    )
                };

                match t.category {
                    RowCategory::TrendingNow => trending.push(item),
                    RowCategory::Top10 => top10.push(item),
                    RowCategory::NewReleases => new_releases.push(item),
                    RowCategory::BecauseYouWatched => because.push(item),
                }
            }

            RootP::new(
                "Netflix",
                vec![
                    Display::text("Welcome back 👋\nPick something to watch."),
                    Display::list(
                        Some("Continue Watching"),
                        continue_watching_items(ctx),
                        None,
                        Some(Flow::new(vec![Screen::new_builder(WatchPage)])),
                    ),
                    Display::list(
                        Some("Trending Now"),
                        trending,
                        None,
                        Some(Flow::new(vec![Screen::new_builder(TitleDetails)])),
                    ),
                    Display::list(
                        Some("Top 10"),
                        top10,
                        None,
                        Some(Flow::new(vec![Screen::new_builder(TitleDetails)])),
                    ),
                    Display::list(
                        Some("New Releases"),
                        new_releases,
                        None,
                        Some(Flow::new(vec![Screen::new_builder(TitleDetails)])),
                    ),
                    Display::list(
                        Some("Because You Watched"),
                        because,
                        None,
                        Some(Flow::new(vec![Screen::new_builder(TitleDetails)])),
                    ),
                ],
                None,
                ("Add Title".into(), Flow::from_form(NewTitleForm)),
                None,
            )
        })
    }

    fn redraw(&mut self, ctx: &mut Context) -> bool {
        let titles = ctx.state.get::<Titles>().unwrap().0.clone();
        (self.cached != titles).then(|| self.cached = titles).is_some()
    }
}

fn continue_watching_items(ctx: &mut Context) -> Vec<ListItem> {
    let progress = ctx.state.get::<ContinueWatching>().unwrap().0.clone();
    let titles = ctx.state.get::<Titles>().unwrap().0.clone();

    progress
        .iter()
        .filter_map(|p| {
            titles.iter().find(|t| t.id == p.title_id).map(|t| {
                let selected = t.clone();
                ListItem::plain(
                    &t.name,
                    &format!("Resume • {}% watched", p.progress_pct),
                    Some("Continue"),
                    move |ctx| {
                        ctx.state.insert(SelectedTitle(selected.clone()));
                    },
                )
            })
        })
        .collect()
}

/* ============================================================
   PAGE: TITLE DETAILS
============================================================ */

#[derive(Debug, Clone)]
pub struct TitleDetails;

impl chk::Page for TitleDetails {
    fn page(&mut self, _: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(|ctx| {
            let t = &ctx.state.get::<SelectedTitle>().unwrap().0;
            let in_list = ctx
                .state
                .get::<MyList>()
                .unwrap()
                .0
                .iter()
                .any(|id| *id == t.id);

            PageType::display(
                &t.name,
                vec![
                    Display::table(
                        "Info",
                        vec![
                            TableItem::new("Type", if t.is_movie { "Movie" } else { "Series" }),
                            TableItem::new("Year", &t.year.to_string()),
                            TableItem::new("Maturity", &t.maturity.to_string()),
                            TableItem::new("Genre", &t.genre.to_string()),
                            TableItem::new("Match", &format!("{}%", t.match_score)),
                            TableItem::new("Length", &format!("{} min", t.duration_min)),
                        ],
                    ),
                    Display::text(&format!("Synopsis\n\n{}", t.description)),
                    Display::actions(vec![
                        ActionItem::new(
                            Action::custom(|ctx| {
                                // Start watching: push into continue watching if not present
                                let selected = ctx.state.get::<SelectedTitle>().unwrap().0.clone();
                                let cw = &mut ctx.state.get_mut_or_default::<ContinueWatching>().0;

                                if !cw.iter().any(|p| p.title_id == selected.id) {
                                    cw.push(WatchingProgress {
                                        title_id: selected.id,
                                        progress_pct: 1,
                                    });
                                }
                            }),
                            "Play",
                            "play",
                        ),
                        ActionItem::new(
                            Action::custom(|ctx| {
                                let selected = ctx.state.get::<SelectedTitle>().unwrap().0.clone();
                                let list = &mut ctx.state.get_mut_or_default::<MyList>().0;

                                if !list.contains(&selected.id) {
                                    list.push(selected.id);
                                }
                            }),
                            if in_list { "In My List" } else { "Add to My List" },
                            "bookmark",
                        ),
                        ActionItem::new(
                            Action::custom(|ctx| {
                                let selected = ctx.state.get::<SelectedTitle>().unwrap().0.clone();
                                let list = &mut ctx.state.get_mut_or_default::<MyList>().0;
                                list.retain(|id| *id != selected.id);
                            }),
                            "Remove from My List",
                            "trash",
                        ),
                    ]),
                    Display::actions(vec![ActionItem::new(
                        Action::flow(Flow::from_form(RateTitleForm)),
                        "Rate this title",
                        "star",
                    )]),
                    Display::actions(vec![ActionItem::new(
                        Action::flow(Flow::new(vec![Screen::new_builder(WatchPage)])),
                        "Open Player",
                        "tv",
                    )]),
                ],
                None,
                Bumper::Done,
                Offset::Start,
            )
        })
    }
}

/* ============================================================
   PAGE: WATCH (FAKE PLAYER)
============================================================ */

#[derive(Debug, Clone)]
pub struct WatchPage;

impl chk::Page for WatchPage {
    fn page(&mut self, _: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(|ctx| {
            let t = &ctx.state.get::<SelectedTitle>().unwrap().0;

            let current_progress = ctx
                .state
                .get::<ContinueWatching>()
                .unwrap()
                .0
                .iter()
                .find(|p| p.title_id == t.id)
                .map(|p| p.progress_pct)
                .unwrap_or(0);

            PageType::display(
                &format!("Watching: {}", t.name),
                vec![
                    Display::text("🎬 Player\n\n(Imagine a video player here)\n"),
                    Display::table(
                        "Playback",
                        vec![
                            TableItem::new("Progress", &format!("{}%", current_progress)),
                            TableItem::new("Quality", "1080p"),
                            TableItem::new("Audio", "English (Stereo)"),
                            TableItem::new("Subtitles", "Off"),
                        ],
                    ),
                    Display::actions(vec![
                        ActionItem::new(
                            Action::custom(|ctx| {
                                let selected = ctx.state.get::<SelectedTitle>().unwrap().0.clone();
                                let cw = &mut ctx.state.get_mut_or_default::<ContinueWatching>().0;

                                if let Some(p) = cw.iter_mut().find(|p| p.title_id == selected.id) {
                                    p.progress_pct = (p.progress_pct.saturating_add(10)).min(100);
                                } else {
                                    cw.push(WatchingProgress {
                                        title_id: selected.id,
                                        progress_pct: 10,
                                    });
                                }
                            }),
                            "Watch +10%",
                            "forward",
                        ),
                        ActionItem::new(
                            Action::custom(|ctx| {
                                let selected = ctx.state.get::<SelectedTitle>().unwrap().0.clone();
                                let cw = &mut ctx.state.get_mut_or_default::<ContinueWatching>().0;

                                if let Some(p) = cw.iter_mut().find(|p| p.title_id == selected.id) {
                                    p.progress_pct = p.progress_pct.saturating_sub(10);
                                }
                            }),
                            "Rewind -10%",
                            "backward",
                        ),
                        ActionItem::new(
                            Action::custom(|ctx| {
                                let selected = ctx.state.get::<SelectedTitle>().unwrap().0.clone();
                                let cw = &mut ctx.state.get_mut_or_default::<ContinueWatching>().0;

                                if let Some(p) = cw.iter_mut().find(|p| p.title_id == selected.id) {
                                    p.progress_pct = 100;
                                }
                            }),
                            "Mark Watched",
                            "checkmark",
                        ),
                    ]),
                ],
                None,
                Bumper::Done,
                Offset::Start,
            )
        })
    }
}

/* ============================================================
   ROOT: SEARCH
============================================================ */

#[derive(Debug, Clone)]
pub struct SearchRoot {
    cached: Vec<Title>,
}

impl SearchRoot {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            cached: ctx.state.get::<Titles>().unwrap().0.clone(),
        }
    }
}

impl chk::Root for SearchRoot {
    fn page(&mut self, _: &mut Context) -> Box<dyn RootBuilder> {
        Box::new(|ctx| {
            let query = ctx
                .state
                .get::<SearchQuery>()
                .map(|q| q.0.clone())
                .unwrap_or_default();

            let titles = ctx.state.get::<Titles>().unwrap().0.clone();

            let results: Vec<ListItem> = titles
                .iter()
                .filter(|t| {
                    query.is_empty()
                        || t.name.to_lowercase().contains(&query.to_lowercase())
                        || t.genre.to_string().to_lowercase().contains(&query.to_lowercase())
                })
                .map(|t| {
                    let selected = t.clone();
                    ListItem::plain(
                        &t.name,
                        &format!("{} • {}", t.year, t.maturity),
                        Some(&t.genre.to_string()),
                        move |ctx|{ ctx.state.insert(SelectedTitle(selected.clone()));},
                    )
                })
                .collect();

            RootP::new(
                "Search",
                vec![
                    Display::text("Search by title or genre."),
                    Display::actions(vec![ActionItem::new(
                        Action::flow(Flow::from_form(SearchForm)),
                        "Search",
                        "search",
                    )]),
                    Display::list(Some("Results"), results, None, Some(Flow::new(vec![
                        Screen::new_builder(TitleDetails)
                    ]))),
                ],
                None,
                ("Clear Query".into(), Flow::new(vec![Screen::new_builder(ClearSearchPage)])),
                None,
            )
        })
    }

    fn redraw(&mut self, ctx: &mut Context) -> bool {
        let titles = ctx.state.get::<Titles>().unwrap().0.clone();
        (self.cached != titles).then(|| self.cached = titles).is_some()
    }
}

#[derive(Debug, Default, Clone)]
pub struct SearchQuery(pub String);

#[derive(Debug, Clone)]
pub struct SearchForm;

impl chk::Form for SearchForm {
    fn inputs(&self) -> Vec<FormItem> {
        vec![
            FormItem::text("Search query", |c| &mut c.state.get_mut_or_default::<SearchQuery>().0),
        ]
    }

    fn success(&self) -> Box<dyn SuccessClosure> {
        Box::new(|ctx| {
            let q = ctx.state.get::<SearchQuery>().unwrap().0.clone();
            ["Search updated".into(), "search".into(), format!("Query: {}", q)]
        })
    }

    fn on_submit(&self, _ctx: &mut Context) {}
}

#[derive(Debug, Clone)]
pub struct ClearSearchPage;

impl chk::Page for ClearSearchPage {
    fn page(&mut self, _: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(|ctx| {
            ctx.state.insert(SearchQuery(String::new()));

            PageType::display(
                "Cleared",
                vec![Display::text("Search query cleared.")],
                None,
                Bumper::Done,
                Offset::Start,
            )
        })
    }
}

/* ============================================================
   ROOT: MY LIST
============================================================ */

#[derive(Debug, Clone)]
pub struct MyListRoot {
    cached: Vec<u32>,
}

impl MyListRoot {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            cached: ctx.state.get::<MyList>().unwrap().0.clone(),
        }
    }
}

impl chk::Root for MyListRoot {
    fn page(&mut self, _: &mut Context) -> Box<dyn RootBuilder> {
        Box::new(|ctx| {
            let titles = ctx.state.get::<Titles>().unwrap().0.clone();
            let list = ctx.state.get::<MyList>().unwrap().0.clone();

            let items: Vec<ListItem> = list
                .iter()
                .filter_map(|id| titles.iter().find(|t| t.id == *id))
                .map(|t| {
                    let selected = t.clone();
                    ListItem::plain(
                        &t.name,
                        &format!("{} • {}", t.year, t.maturity),
                        Some(&t.genre.to_string()),
                        move |ctx| {ctx.state.insert(SelectedTitle(selected.clone()));},
                    )
                })
                .collect();

            RootP::new(
                "My List",
                vec![
                    Display::text("Your saved titles."),
                    Display::list(None, items, None, Some(Flow::new(vec![
                        Screen::new_builder(TitleDetails)
                    ]))),
                ],
                None,
                ("Clear My List".into(), Flow::new(vec![Screen::new_builder(ClearMyListPage)])),
                None,
            )
        })
    }

    fn redraw(&mut self, ctx: &mut Context) -> bool {
        let list = ctx.state.get::<MyList>().unwrap().0.clone();
        (self.cached != list).then(|| self.cached = list).is_some()
    }
}

#[derive(Debug, Clone)]
pub struct ClearMyListPage;

impl chk::Page for ClearMyListPage {
    fn page(&mut self, _: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(|ctx| {
            ctx.state.insert(MyList(vec![]));
            PageType::display(
                "My List",
                vec![Display::text("Cleared your list.")],
                None,
                Bumper::Done,
                Offset::Start,
            )
        })
    }
}

/* ============================================================
   FORM: RATE TITLE
============================================================ */

#[derive(Debug, Default, Clone)]
pub struct NewRating {
    pub rating: String,
    pub review: String,
}

#[derive(Debug, Clone)]
pub struct RateTitleForm;

impl chk::Form for RateTitleForm {
    fn inputs(&self) -> Vec<FormItem> {
        vec![
            FormItem::number("Rating (1-5)", NumberVariant::Currency, |c| {
                &mut c.state.get_mut_or_default::<NewRating>().rating
            }),
            FormItem::text("Review (optional)", |c| {
                &mut c.state.get_mut_or_default::<NewRating>().review
            }),
        ]
    }

    fn success(&self) -> Box<dyn SuccessClosure> {
        Box::new(|ctx| {
            let t = &ctx.state.get::<SelectedTitle>().unwrap().0;
            let r = ctx.state.get::<NewRating>().unwrap().rating.to_string();
            ["Thanks!".into(), "star".into(), format!("You rated {}: {}/5", t.name, r)]
        })
    }

    fn on_submit(&self, ctx: &mut Context) {
        // Backend idea:
        // POST /titles/{id}/rating
        // For now: no persistence
        let _t = ctx.state.get::<SelectedTitle>().unwrap().0.clone();
        let _rating = ctx.state.get::<NewRating>().unwrap().clone();
    }
}

/* ============================================================
   FORM: ADD NEW TITLE
============================================================ */

#[derive(Debug, Default, Clone)]
pub struct NewTitle {
    pub name: String,
    pub description: String,
    pub year: String,
    pub duration_min: String,
    pub match_score: String,
    pub is_movie: bool,
    pub maturity: Maturity,
    pub genre: Genre,
    pub category: RowCategory,
}

#[derive(Debug, Clone)]
pub struct NewTitleForm;

impl chk::Form for NewTitleForm {
    fn inputs(&self) -> Vec<FormItem> {
        vec![
            FormItem::text("Title name", |c| &mut c.state.get_mut_or_default::<NewTitle>().name),
            FormItem::text("Description", |c| &mut c.state.get_mut_or_default::<NewTitle>().description),
            FormItem::number("Year", NumberVariant::Currency, |c| &mut c.state.get_mut_or_default::<NewTitle>().year),
            FormItem::number("Duration (min)", NumberVariant::Currency, |c| {
                &mut c.state.get_mut_or_default::<NewTitle>().duration_min
            }),
            FormItem::number("Match score (0-100)", NumberVariant::Currency, |c| {
                &mut c.state.get_mut_or_default::<NewTitle>().match_score
            }),
            FormItem::enumerator("Type", vec![
                EnumItem::new("Movie", "Single title", |c| c.state.get_mut_or_default::<NewTitle>().is_movie = true),
                EnumItem::new("Series", "Multiple episodes", |c| c.state.get_mut_or_default::<NewTitle>().is_movie = false),
            ]),
            FormItem::enumerator("Maturity", vec![
                EnumItem::new("G", "General audiences", |c| c.state.get_mut_or_default::<NewTitle>().maturity = Maturity::G),
                EnumItem::new("PG", "Parental guidance", |c| c.state.get_mut_or_default::<NewTitle>().maturity = Maturity::PG),
                EnumItem::new("PG-13", "Teens and up", |c| c.state.get_mut_or_default::<NewTitle>().maturity = Maturity::PG13),
                EnumItem::new("R", "Restricted", |c| c.state.get_mut_or_default::<NewTitle>().maturity = Maturity::R),
                EnumItem::new("TV-PG", "TV parental guidance", |c| c.state.get_mut_or_default::<NewTitle>().maturity = Maturity::TVPG),
                EnumItem::new("TV-14", "14+", |c| c.state.get_mut_or_default::<NewTitle>().maturity = Maturity::TV14),
                EnumItem::new("TV-MA", "Mature audiences", |c| c.state.get_mut_or_default::<NewTitle>().maturity = Maturity::TVMA),
            ]),
            FormItem::enumerator("Genre", vec![
                EnumItem::new("Action", "Fast paced", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::Action),
                EnumItem::new("Comedy", "Funny", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::Comedy),
                EnumItem::new("Drama", "Serious", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::Drama),
                EnumItem::new("Documentary", "Real stories", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::Documentary),
                EnumItem::new("Horror", "Scary", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::Horror),
                EnumItem::new("Romance", "Love", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::Romance),
                EnumItem::new("Reality", "Unscripted", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::Reality),
                EnumItem::new("Sci-Fi", "Futuristic", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::SciFi),
                EnumItem::new("Kids", "Family friendly", |c| c.state.get_mut_or_default::<NewTitle>().genre = Genre::Kids),
            ]),
            FormItem::enumerator("Home Row", vec![
                EnumItem::new("Trending Now", "Popular today", |c| c.state.get_mut_or_default::<NewTitle>().category = RowCategory::TrendingNow),
                EnumItem::new("Top 10", "Most watched", |c| c.state.get_mut_or_default::<NewTitle>().category = RowCategory::Top10),
                EnumItem::new("New Releases", "Fresh content", |c| c.state.get_mut_or_default::<NewTitle>().category = RowCategory::NewReleases),
                EnumItem::new("Because You Watched", "Personalized", |c| c.state.get_mut_or_default::<NewTitle>().category = RowCategory::BecauseYouWatched),
            ]),
        ]
    }

    fn success(&self) -> Box<dyn SuccessClosure> {
        Box::new(|ctx| {
            let t = ctx.state.get::<NewTitle>().unwrap();
            ["Title added".into(), "plus".into(), format!("Created {}", t.name)]
        })
    }

    fn on_submit(&self, ctx: &mut Context) {
        let new = ctx.state.get::<NewTitle>().unwrap().clone();

        let titles = &mut ctx.state.get_mut_or_default::<Titles>().0;
        let next_id = titles.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        titles.push(Title {
            id: next_id,
            name: new.name,
            description: new.description,
            year: new.year,
            maturity: new.maturity,
            genre: new.genre,
            category: new.category,
            duration_min: new.duration_min,
            match_score: new.match_score,
            is_movie: new.is_movie,
        });
    }
}
