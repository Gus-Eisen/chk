use chk::*;

pub struct Orange;
impl chk::Application for Orange {
    fn roots(ctx: &mut Context) -> Vec<Root> {
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
        vec![Root::new(RootContent::icon("wallet"), BitcoinHome::build(ctx))]
    }

    fn theme() -> Theme { Theme::Dark(Color::from_hex("#eb343a", 255)) }
    // fn on_event() -> Box<dyn FnMut(&mut Context, Box<dyn Event>) -> Vec<Box<dyn Event>>>;
}

chk::run!(Orange);

// if event.downcast_ref::<TickEvent>().is_some() {
//     if let Some(tx) = ctx.state().get_mut::<NewTransaction>() {
//         if let Some(usd_value) = tx.inner.amount.usd() {
//             tx.inner.amount.btc = format!("{:.8} BTC", usd_value / 1_000_000_000.00);
//         }
//     }

//     ctx.state().get_named::<String>("AddressTextInput").cloned().and_then(|address| {
//         ctx.state().get_mut::<NewTransaction>().map(|tx| tx.inner.address = address)
//     });

//     ctx.state().get_named::<String>("AmountCurrencyInput").cloned().and_then(|amt| {
//         ctx.state().get_mut::<NewTransaction>().map(|tx| tx.inner.amount.usd = amt)
//     });

//     ctx.state().get_named::<String>("FeeEnumerator").cloned().and_then(|val| {
//         ctx.state().get_mut::<NewTransaction>().map(|tx| tx.inner.is_priority = val == "Priority")
//     });
// }


#[derive(Debug, Clone)]
pub struct BitcoinHome; //(TestState); 

// impl chk::Root for BitcoinHome {
//     fn name() -> String {"Wallet".to_string()}
//     fn redraw(&self, ctx: &mut Context) -> bool {ctx.state.get_or_default::<TestState>() != self.0}
// }

impl BitcoinHome {
    fn build(ctx: &mut Context) -> RootPage {
        let transactions = ctx.state.get::<MyTransactions>().unwrap().inner.clone();
        RootPage::new("Wallet", 
            vec![
                Display::currency(12.56, "0.00001234 BTC"),
                Display::list(None, transactions.into_iter().map(|tx| {
                    let dir = if tx.is_received {"Received"} else {"Sent"};
                    ListItem::plain(&format!("Bitcoin {dir}"), &tx.amount.btc, Some(&tx.amount.usd), &tx.txid)
                }).collect::<Vec<ListItem>>(), Some(ViewTransaction::build()), None)
            ], 
            None,
            RootBumper::new("Receive", Receive::build()),
            Some(RootBumper::new("Send", Send::build())),
        )
    }

    // fn on_event(&self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {vec![event]}
}


pub struct Receive;
impl Receive {
    pub fn build() -> Flow {
        let address = "staesuh8438iy92i984did48i";
        Flow::new(vec![Box::new(|_state: &mut State| PageType::display("Receive bitcoin", 
            vec![Display::qr_code(address, "Scan to receive bitcoin.")], 
            None, Bumper::custom("Share", Action::share(address), None), Offset::Center
        ))])
    }
}

pub struct ViewTransaction;
impl ViewTransaction {
    pub fn build() -> Flow {
        Flow::new(vec![Box::new(|state: &mut State| {
            let tx = &state.get::<NewTransaction>().unwrap().inner;
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
        })])
    }
}

pub struct Send;
impl Send {
    pub fn build() -> Flow {
        let address = |_state: &mut State| PageType::input("Bitcoin address", 
            Input::text("Bitcoin address", false, None, |ctx: &mut Context, val: &mut String| {
                ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.address = val.to_string());
            }), 
            Bumper::default(Some(Box::new(|ctx: &mut Context| {
                ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.address.is_empty()).unwrap_or_default()
            })))
        );
        // Some(vec![
            // QuickAction::custom("Paste Clipboard", "Pasted", |_ctx: &mut Context| {}),
            // QuickAction::flow("Scan QR Code", ScanQRCode::new()),
            // QuickAction::flow("Select Contact", SelectContact::new())
        // ])

        let amount = |_state: &mut State| PageType::input("Bitcoin amount", 
            Input::currency("Enter send amount", |ctx: &mut Context, val: &mut String| {
                ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.amount.usd = val.to_string());
            }), 
            Bumper::default(Some(Box::new(|ctx: &mut Context| false)))
        );

        let speed = |_state: &mut State| PageType::input("Transaction speed", 
            Input::enumerator(vec![
                EnumItem::new("Standard", "Arrives in ~2 hours\n$0.18 bitcoin network fee", |ctx: &mut Context| {
                    ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.is_priority = false);
                }),
                EnumItem::new("Priority", "Arrives in ~30 minutes\n$0.32 bitcoin network fee", |ctx: &mut Context| {
                    ctx.state.get_mut::<NewTransaction>().map(|tx| tx.inner.is_priority = true);
                }),
            ]), 
            Bumper::default(None)
        );

        let review = |state: &mut State| {
            let tx = &state.get::<NewTransaction>().unwrap().inner;
            let speed = if tx.is_priority {"Priority (~30 mins)"} else {"Standard (~2 hr)"};
            PageType::review("Confirm send", vec![
                Display::review("Confirm address", &tx.address, "Bitcoin sent to the wrong address can never be recovered."),
                Display::table("Confirm amount", vec![
                    TableItem::new("Amount Sent (BTC)", &tx.amount.btc),
                    TableItem::new("Amount Sent", &tx.amount.usd),
                    TableItem::new("Transaction Speed", speed),
                    TableItem::new("Transaction Fee", &tx.fee),
                    TableItem::new( "Transaction Total", &tx.total),
                ])
            ])
        };

        let success = |_state: &mut State| PageType::success("Bitcoin sent", "bitcoin", "You sent $10.00");

        let on_submit = |ctx: &mut Context| {
            let tran = ctx.state.get::<NewTransaction>().unwrap().clone();
            println!("Broadcasting transaction... {:?}", tran);
            ctx.state.get_mut_or_default::<MyTransactions>().inner.push(tran.inner);
        };
        Flow::form(vec![Box::new(address), Box::new(amount), Box::new(speed)], Some(Box::new(review)), Box::new(success), on_submit)
    }
}


#[derive(Clone, Debug, Default)]    
pub struct BitcoinAmount {
    pub btc: String,
    pub usd: String,
}

impl BitcoinAmount {
    pub fn usd(&self) -> Option<f32> {
        self.usd.trim_start_matches('$').replace(',', "").parse::<f32>().ok()
    }
}

#[derive(Clone, Debug, Default)]
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
