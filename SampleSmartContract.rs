#[macro_use]
extern crate pwasm;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use pwasm::{Contract, ContractDef};

#[derive(Serialize, Deserialize)]
pub struct State {
  current: U256,
}

contract!(foo_contract);

fn foo_contract() -> impl ContractDef<State> {
  messages! {
    Add(U256);
    Get() -> U256;
  }

  Contract::new()
    .constructor(|_txdata, initial| State {
      current: initial,
    })
    // These type annotations aren't necessary, they're just
    // to show that we don't receive a mutable reference to the
    // state in the `Get` method.
    .on_msg_mut::<Add>(|_env: &mut EthEnv, state: &mut State, to_add| {
      state.current += to_add;
    })
    .on_msg::<Get>(|_env: &EthEnv, state: &State, ()| state.current.clone())
}
