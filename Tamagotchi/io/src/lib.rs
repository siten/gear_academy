#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata};
use gstd::{msg, prelude::*, ActorId};
use scale_info::TypeInfo;

pub type AttributeId = u32;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
   type Init = InOut<String,()>;
   type Reply = InOut<(),()>;
   type Others = InOut<(),()>;
   type Signal = ();
   type Handle = InOut<TmgAction, TmgEvent>;
   type State = Tamagotchi;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
   //1
   Name,
   Age,
   //2
   Feed,
   Play,
   Sleep,
   //3
   Transfer(ActorId),
   Approve(ActorId),
   RevokeApproval,
   //4
   ApproveTokens {
      account: ActorId,
      amount: u128,
  },
  SetFTokenContract(ActorId),
  BuyAttribute {
      store_id: ActorId,
      attribute_id: AttributeId,
  },
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
   //1
   Name(String),
   Age(u64),
   //2
   Fed,
   Entertained,
   Slept,
   //3
   Transfer(ActorId),
   Approve(ActorId),
   RevokeApproval,
   //4
   ApproveTokens { account: ActorId, amount: u128 },
   ApprovalError,
   SetFTokenContract,
   AttributeBought(AttributeId),
   CompletePrevPurchase(AttributeId),
   ErrorDuringPurchase,
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
   //1
   name: String,
   date_of_birth: u64,
   //2
   pub owner: ActorId,
   pub fed: u64,
   pub fed_block: u64,
   pub entertained: u64,
   pub entertained_block: u64,
   pub rested: u64,
   pub rested_block: u64,
   //3
   pub allowed_account: Option<ActorId>,
   //4
   pub ft_contract_id: Option<ActorId>,

}

impl Tamagotchi {
   pub fn new(name: String, date_of_birth: u64, creator:ActorId) -> Self {
      let cur_block = gstd::exec::block_timestamp();

      Tamagotchi {
           //1
           name: name,
           date_of_birth: date_of_birth,
           //2
           owner:creator,
           fed:10000,
           fed_block:cur_block,

           entertained:10000,
           entertained_block:cur_block,

           rested:10000,
           rested_block:cur_block,
           //3
           allowed_account:Some(creator),
           //4
           ft_contract_id:None,
       }
   }

   // ActorId
   //id[0..8].copy_from_slice(&other.to_le_bytes()[..]); //other u64
   //id.into()

   pub fn get_name(&self) -> String{
      self.name.clone()
   }

   pub fn get_age(&self) -> u64{
      self.date_of_birth
   }

   //2
   pub fn feed(&mut self) {
      let cur_block = gstd::exec::block_timestamp();
      if self.fed_block < cur_block {
         let hunger_consumption = (cur_block - self.fed_block) * HUNGER_PER_BLOCK;
         if hunger_consumption >= self.fed {
            self.fed = 0;
         }
         else {
            self.fed = self.fed - hunger_consumption;
         }

         self.fed = self.fed + FILL_PER_FEED;
         self.fed_block = cur_block;
      }
   }
   pub fn play(&mut self){
      let cur_block = gstd::exec::block_timestamp();
      if self.entertained_block < cur_block {
         let boredom = (cur_block - self.entertained_block) * BOREDOM_PER_BLOCK ;
         if boredom >= self.entertained {
            self.entertained = 0;
         }
         else {
            self.entertained = self.entertained - boredom;
         }

         self.entertained = self.entertained + FILL_PER_ENTERTAINMENT;
         self.entertained_block = cur_block;
      }
   }
   pub fn sleep(&mut self){
      let cur_block = gstd::exec::block_timestamp();
      if self.rested_block < cur_block {
          // TODO

          self.rested_block = cur_block;
      }
   }

   //3
   pub fn transfer(&mut self, actorid:ActorId){
      assert_eq!(
         msg::source(),
         self.owner,
         "The message sender must be owner"
         );

         self.owner = actorid;
   }
   pub fn approve(&mut self, actorid:ActorId){
      assert_eq!(
         msg::source(),
         self.owner,
         "The message sender must be owner"
         );

         self.allowed_account = Some(actorid);
   }
   pub fn revokeapproval(&mut self){
      assert_eq!(
         msg::source(),
         self.owner,
         "The message sender must be owner"
         );

         self.allowed_account = None;
   }

   //4
   pub fn approvetokens(&mut self, account: ActorId, amount: u128){
      assert_eq!(
         msg::source(),
         self.owner,
         "Only owner can approvetokens"
     );
     //TODO
     //像FT合约发消息 授权 account 可以转移
   }
   pub fn setftokencontract(&mut self, actorid:ActorId){
      assert_eq!(
         msg::source(),
         self.owner,
         "Only owner can set fungible token contract"
     );
     self.ft_contract_id = Some(actorid);
     msg::reply(
      TmgEvent::SetFTokenContract,
         0,
     )
     .expect("Error in sending a reply `TmgEvent::SetFTokenContract`");
   }
   pub fn buyattribute(&mut self, store_id: ActorId, attribute_id: AttributeId){
      assert_eq!(
         msg::source(),
         self.owner,
         "Only owner can buy attribute"
     );
     //TODO 向商店合约发送购买消息
     //msg::send(store_id, , 0)
   }

}



static HUNGER_PER_BLOCK:u64 = 1; // Tamagotchi 对这个区块的饥饿程度；
static ENERGY_PER_BLOCK:u64 = 2; // Tamagotchi 每个区块损失多少能量；
static BOREDOM_PER_BLOCK:u64 = 2; // Tamagotchi 每个区块的无聊程度；
static FILL_PER_SLEEP:u64 = 1000; // Tamagotchi 每次睡眠获得多少能量；
static FILL_PER_FEED:u64 = 1000; // Tamagotchi 在喂食期间吃饱的量；
static FILL_PER_ENTERTAINMENT:u64 = 1000; // Tamagotchi 在喂食期间变得快乐的程度；
