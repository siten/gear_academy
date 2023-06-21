#![no_std]
use gstd::{debug, msg, prelude::*};
use io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
unsafe extern "C" fn handle() {
    let input_message: TmgAction = msg::load().expect("Error in loading InputMessages");

    let tamagotchi_ref = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };

    // let tamagotchi_ref = Tamagotchi::new(String::from("none"),0);

    // let tamagotchi = match &TAMAGOTCHI {
    //     Some(s) => s,
    //     None => &default_obj,
    // };

    match input_message {
        //1
        TmgAction::Name => {
            //debug!("Message: SendHelloTo {:?} msg {}", account, greeting);
            msg::reply(TmgEvent::Name(tamagotchi_ref.get_name()), 0)
                .expect("Error in sending name to account");
        }
        TmgAction::Age => {
            //debug!("Message: SendHelloReply");
            msg::reply(TmgEvent::Age(tamagotchi_ref.get_age()), 0)
                .expect("Error in sending age to account");
        }
        //2
        TmgAction::Feed => {
            tamagotchi_ref.feed();
        }
        TmgAction::Play => {
            tamagotchi_ref.play();
        }
        TmgAction::Sleep => {
            tamagotchi_ref.sleep();
        }
        //3
        TmgAction::Transfer(actorid) => {
            tamagotchi_ref.transfer(actorid);
        }
        TmgAction::Approve(actorid) => {
            tamagotchi_ref.approve(actorid);
        }
        TmgAction::RevokeApproval => {
            tamagotchi_ref.revokeapproval();
        }
        //4
        TmgAction::ApproveTokens { account, amount } => {
            tamagotchi_ref.approvetokens(account, amount);
        }
        TmgAction::SetFTokenContract(actorid) => {
            tamagotchi_ref.setftokencontract(actorid);
        }
        TmgAction::BuyAttribute {
            store_id,
            attribute_id,
        } => {
            tamagotchi_ref.buyattribute(store_id, attribute_id);
        }
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_message = msg::load().expect("Can't load init message");

    //debug!("Program was initialized with message {:?}", greeting);
    let tamagotchi = Tamagotchi::new(init_message, 0, msg::source());
    TAMAGOTCHI = Some(tamagotchi);
}

#[no_mangle]
extern "C" fn state() {
    msg::reply("none", 0).expect("Failed to share state");
}
