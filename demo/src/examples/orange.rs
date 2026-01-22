use chk::{
    RootInfo, FormItem, NumberVariant, SuccessClosure, Flow, Bumper, ActionItem,
    RootP, Display, RootBuilder, Offset, Context, Screen, PageType, PageBuilder,
    Color, Theme
};
use chk::items::{ListItem, Action, EnumItem, TableItem};

use std::sync::Arc;
use image::RgbaImage;

#[derive(Debug, Default)]
pub struct ImagePath(String);

pub struct Orange;
impl chk::App for Orange {
    fn roots(&self, ctx: &mut Context) -> Vec<RootInfo> {
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
        
        let home = BitcoinHome::new(ctx);
        vec![RootInfo::icon("wallet", "Test2", Box::new(Screen::new(ctx, home)))]
    }

    fn theme(&self) -> Theme {Theme::Dark(Color::from_hex("#EB343A", 255))}
}


#[derive(Debug, Clone)]
pub struct BitcoinHome(Vec<Transaction>);
impl BitcoinHome {
    pub fn new(ctx: &mut Context) -> Self {
        BitcoinHome(ctx.state.get::<MyTransactions>().unwrap().inner.clone())
    }
}

impl chk::Root for BitcoinHome {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn RootBuilder> {
        Box::new(move |ctx: &mut Context| {
            let transactions = ctx.state.get::<MyTransactions>().unwrap().inner.clone();
            let receive = Screen::new_builder(Receive);
            RootP::new(
                "Test", 
                vec![
                    Display::currency(12.56, "0.00001234 BTC"),
                    Display::list(None, transactions.into_iter().map(|tx| {
                        let dir = if tx.is_received {"Received"} else {"Sent"};
                        ListItem::plain(&format!("Bitcoin {dir}"), &tx.amount.btc, Some(&tx.amount.usd), |ctx: &mut Context| {})
                    }).collect::<Vec<ListItem>>(), None, Some(Flow::new(vec![Screen::new_builder(ViewTransaction)])))
                ], 
                None, 
                ("Receive".to_string(), Flow::new(vec![receive])), 
                Some(("Send".to_string(), Flow::from_form(Send)))
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

#[derive(Debug, Clone)]
pub struct ViewTransaction;
impl chk::Page for ViewTransaction {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        Box::new(move |ctx: &mut Context| {
            let tx = &ctx.state.get::<NewTransaction>().unwrap().inner;
            let dir = if tx.is_received {"Received"} else {"Sent"};
            PageType::display(&format!("{dir} bitcoin"), vec![
                Display::currency(12.56, "0.00001234 BTC"),
                Display::table("Transcation details", vec![
                    TableItem::new("Amount Sent (BTC)", &tx.amount.btc),
                    TableItem::new("Amount Sent", &tx.amount.usd),
                    TableItem::new("Transaction Fee", &tx.fee),
                    TableItem::new( "Transaction Total", &tx.total),
                ])
            ], None, Bumper::Done, Offset::Start)
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

#[derive(Clone)]
pub struct Send;
impl chk::Form for Send {
    fn inputs(&self) -> Vec<FormItem> {vec![
        FormItem::text("Bitcoin address", |ctx: &mut Context| &mut ctx.state.get_mut_or_default::<NewTransaction>().inner.address),
        FormItem::number("Bitcoin amount", NumberVariant::Currency, |ctx: &mut Context| &mut ctx.state.get_mut_or_default::<NewTransaction>().inner.amount.usd),
        FormItem::enumerator("TransactionSpeed", vec![
            EnumItem::new("Standard", "Arrives in ~2 hours\n$0.18 bitcoin network fee", |ctx: &mut Context| {
                ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.is_priority = false);
            }),
            EnumItem::new("Priority", "Arrives in ~30 minutes\n$0.32 bitcoin network fee", |ctx: &mut Context| {
                ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.is_priority = true);
            }),
        ])
    ]}

    fn review(&self) -> Option<Box<dyn chk::ReviewPage>> { Some(Box::new(TransactionReview)) }
    fn success(&self) -> Box<dyn SuccessClosure> {Box::new(|ctx: &mut Context| {
        let tx = &ctx.state.get::<NewTransaction>().unwrap().inner;
        ["Transaction Sent".to_string(), "bitcoin".to_string(), format!("You sent {}", tx.amount.usd)]
    })}

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
