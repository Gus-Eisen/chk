use chk::{Success, Flow, Bumper, RootP, Display, RootBuilder, layout::Offset, Context, drawable::{Drawable, Component}, Screen, components::Circle, event::{OnEvent, Event}, layout::Stack, PageType, PageBuilder, ScreenBuilder};
use chk::components::interface::{Interface, RootInfo};
use ramp::prism;

use chk::items::{ListItem, Action, Input, EnumItem, TableItem};

use std::sync::Arc;
use image::RgbaImage;

#[derive(Debug, Default)]
pub struct ImagePath(String);

#[derive(Debug, Clone)]
pub struct BitcoinHome(Vec<Transaction>);
impl BitcoinHome {
    pub fn new(ctx: &mut Context) -> Self {BitcoinHome(ctx.state.get::<MyTransactions>().unwrap().inner.clone())}
}
impl chk::Root for BitcoinHome {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn RootBuilder> {
        Box::new(move |ctx: &mut Context| {
            let transactions = ctx.state.get::<MyTransactions>().unwrap().inner.clone();
            let receive = Screen::new_builder(ctx, Receive);
            RootP::new(
                "Test", 
                vec![
                    Display::currency(12.56, "0.00001234 BTC"),
                    Display::list(None, transactions.into_iter().map(|tx| {
                        let dir = if tx.is_received {"Received"} else {"Sent"};
                        ListItem::plain(&format!("Bitcoin {dir}"), &tx.amount.btc, Some(&tx.amount.usd), &tx.txid)
                    }).collect::<Vec<ListItem>>(), None) // Some(Flow::new(vec![Box::new(ViewTransaction::new(ctx))]))
                ], 
                None, 
                ("Receive".to_string(), Flow::new(vec![receive])), 
                Some(("Send".to_string(), Flow::from_form(ctx, Send)))
            )
        })
    }

    fn redraw(&mut self, ctx: &mut Context) -> bool {
        let transactions = ctx.state.get::<MyTransactions>().unwrap().inner.clone();
        if self.0 != transactions {
            self.0 = transactions;
            return true;
        }
        false
    }
}

pub struct Orange;
impl Orange {
    pub fn new(ctx: &mut Context) -> Interface {
        ctx.state.insert(NewTransaction::default());
        ctx.state.insert(MyTransactions {
            inner: vec![
                Transaction::new(true, "$12.30"),
                Transaction::new(true, "$142.10"),
                Transaction::new(false, "$46.30"),
                Transaction::new(true, "$28.65"),
                Transaction::new(false, "$22.31"),
            ]
        });
        ctx.state.insert(ImagePath("seagull".to_string()));
        let home = BitcoinHome::new(ctx);
        let page = Screen::new(ctx, home);
        Interface::new(ctx, 
            vec![RootInfo::icon("wallet", "Test2", Box::new(page))], 
            Box::new(|ctx: &mut Context, event: Box<dyn Event>| {vec![event]})
        )
    }
}

ramp::run!{|ctx: &mut Context| Orange::new(ctx)}

// pub struct Orange;
// impl chk::Application for Orange {
//     fn roots(ctx: &mut Context) -> Vec<Box<dyn Root>> {
//         ctx.state.insert(NewTransaction::default());
//         ctx.state.insert(MyTransactions {
//             inner: vec![
//                 Transaction::new(true, "$12.30"),
//                 Transaction::new(true, "$142.10"),
//                 Transaction::new(false, "$46.30"),
//                 Transaction::new(true, "$28.65"),
//                 Transaction::new(false, "$22.31"),
//             ]
//         });
//         vec![BitcoinHome::new(ctx)]
//     }

//     fn theme() -> Theme { Theme::Dark(Color::from_hex("#eb343a", 255)) }
//     // fn on_event() -> Box<dyn FnMut(&mut Context, Box<dyn Event>) -> Vec<Box<dyn Event>>>;
// }

// chk::run!(Orange);


#[derive(Debug, Clone)]
pub struct Receive;
impl chk::Page for Receive {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(move |ctx: &mut Context| {
            let address = Address::generate();
            PageType::display("Receive bitcoin", 
                vec![Display::qr_code(&address, "Scan to receive bitcoin.")], 
                None, Bumper::custom("Share", Action::share(&address), None), Offset::Center
            )
        })
    }
}

// #[derive(Debug, Clone)]
// pub struct ViewTransaction(PageType);
// impl ViewTransaction {
//     pub fn new(ctx: &mut Context) -> Self {
//         let tx = &ctx.state.get::<NewTransaction>().unwrap().inner;
//         let dir = if tx.is_received {"Received"} else {"Sent"};
//         ViewTransaction(PageType::display(&format!("{dir} bitcoin"), vec![
//             Display::currency(12.56, "0.00001234 BTC"),
//             Display::table("Transcation details", vec![
//                 TableItem::new("Amount Sent (BTC)", &tx.amount.btc),
//                 TableItem::new("Amount Sent", &tx.amount.usd),
//                 TableItem::new("Transaction Fee", &tx.fee),
//                 TableItem::new( "Transaction Total", &tx.total),
//             ])
//         ], None, Bumper::Done, Offset::Start))
//     }
// }

// impl chk::Page for ViewTransaction {
//     fn page(&mut self, ctx: &mut Context) -> &mut PageType {&mut self.0}
//     fn redraw(&mut self, ctx: &mut Context) -> bool {
//         println!("CHECKING TO REDRAW");
//         true
//     }
// }

#[derive(Debug, Clone)]
pub struct TransactionAddress;
impl chk::Page for TransactionAddress {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        // Some(vec![
        //     QuickAction::custom("Paste Clipboard", "Pasted", |_ctx: &mut Context| {}),
        //     QuickAction::flow("Scan QR Code", ScanQRCode::new()),
        //     QuickAction::flow("Select Contact", SelectContact::new())
        // ])

        Box::new(move |ctx: &mut Context| {
            PageType::input("Bitcoin address", 
                Input::text("Bitcoin address", false, None, |ctx: &mut Context, val: &mut String| {
                    ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.address = val.to_string());
                }), None,
                Bumper::default(Some(Box::new(|ctx: &mut Context| {
                    ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.address.is_empty()).unwrap_or_default()
                })))
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct TransactionAmount;
impl chk::Page for TransactionAmount {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(move |ctx: &mut Context| {
            PageType::input("Bitcoin amount", 
                Input::currency("Enter send amount", |ctx: &mut Context, val: &mut String| {
                    ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.amount.usd = val.to_string());
                }), None,
                Bumper::default(Some(Box::new(|ctx: &mut Context| false)))
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct TransactionSpeed;
impl chk::Page for TransactionSpeed {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(move |ctx: &mut Context| {
            PageType::input("Transaction speed", 
                Input::enumerator(vec![
                    EnumItem::new("Standard", "Arrives in ~2 hours\n$0.18 bitcoin network fee", |ctx: &mut Context| {
                        ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.is_priority = false);
                    }),
                    EnumItem::new("Priority", "Arrives in ~30 minutes\n$0.32 bitcoin network fee", |ctx: &mut Context| {
                        ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.is_priority = true);
                    }),
                ]), None,
                Bumper::default(None)
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct TransactionReview;
impl chk::ReviewPage for TransactionReview {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(move |ctx: &mut Context| {
            let tx = &ctx.state.get::<NewTransaction>().unwrap().inner;
            let speed = if tx.is_priority {"Priority (~30 mins)"} else {"Standard (~2 hr)"};
            PageType::display("Confirm send", 
                vec![
                    Display::review("Confirm address", &tx.address, "Bitcoin sent to the wrong address can never be recovered."),
                    Display::table("Confirm amount", vec![
                        TableItem::new("Amount Sent (BTC)", &tx.amount.btc),
                        TableItem::new("Amount Sent", &tx.amount.usd),
                        TableItem::new("Transaction Speed", speed),
                        TableItem::new("Transaction Fee", &tx.fee),
                        TableItem::new( "Transaction Total", &tx.total),
                    ])
                ], None, Bumper::default(None), Offset::Start
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct TransactionSuccess;
impl chk::SuccessPage for TransactionSuccess {
    fn info(&mut self, ctx: &mut Context) -> [String; 3] {
        let tx = &ctx.state.get::<NewTransaction>().unwrap().inner;
        ["Transaction Sent".to_string(), "bitcoin".to_string(), format!("You sent {}", tx.amount.usd)]
    }
}

#[derive(Clone)]
pub struct Send;
impl chk::Form for Send {
    fn inputs(&self) -> Vec<Box<dyn chk::Page>> {
        vec![Box::new(TransactionAddress), Box::new(TransactionAmount), Box::new(TransactionSpeed)]
    }

    fn review(&self) -> Option<Box<dyn chk::ReviewPage>> { Some(Box::new(TransactionReview)) }
    fn success(&self) -> Box<dyn chk::SuccessPage> { Box::new(TransactionSuccess) }

    fn on_submit(&self, ctx: &mut Context) {
        let tran = ctx.state.get::<NewTransaction>().unwrap().clone();
        println!("Broadcasting transaction... {:?}", tran);
        ctx.state.get_mut_or_default::<MyTransactions>().inner.push(tran.inner);
    }
}

pub struct Address;
impl Address {
    pub fn generate() -> String { "bcp1ceax843sTOhuad2lahteau29uxxTHoxalo".to_string() }
}

#[derive(Clone, Debug, Default, PartialEq)]    
pub struct BitcoinAmount {
    pub btc: String,
    pub usd: String,
}

impl BitcoinAmount {
    pub fn usd(&self) -> Option<f32> {
        self.usd.trim_start_matches('$').replace(',', "").parse::<f32>().ok()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Transaction {
    pub address: String,
    pub amount: BitcoinAmount,
    pub is_priority: bool,
    pub fee: String,
    pub total: String,
    pub is_received: bool,
    pub txid: String,
}

impl Transaction {
    pub fn new(is_received: bool, usd: &str) -> Self {
        Transaction {
            address: "bcp1ceax843sTOhuad2lahteau29uxxTHoxalo".to_string(),
            amount: BitcoinAmount {
                btc: "0.00001234 BTC".to_string(),
                usd: usd.to_string(),
            },
            is_priority: !is_received,
            fee: "$3.00".to_string(),
            total: "$536.99".to_string(),
            is_received,
            txid: "TXID".to_string()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NewTransaction {
    pub inner: Transaction
}

#[derive(Clone, Debug, Default)]
pub struct MyTransactions {
    pub inner: Vec<Transaction>
}
