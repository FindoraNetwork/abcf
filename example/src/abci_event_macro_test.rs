use abcf::Event;
use serde::{Deserialize, Serialize};
use tm_protos::abci;

#[derive(Clone, Debug, Deserialize, Serialize, macros::Event)]
pub struct E {
    #[abcf(index)]
    age: u8,
    name: String,
    height: u8,
    #[abcf(index)]
    weight: u8,
}

#[test]
fn test() {
    let e = E {
        age: 30,
        name: "jack".to_string(),
        height: 190,
        weight: 78,
    };

    let abci_event: abci::Event = e.to_abci_event();

    assert_eq!(e.name(), "E");
    assert_eq!(abci_event.attributes[0].index, true);
    assert_eq!(abci_event.attributes[3].index, true);
    assert_eq!(
        abci_event.attributes[0].key,
        serde_json::to_vec(&*"age").unwrap()
    );
    assert_eq!(
        abci_event.attributes[0].value,
        serde_json::to_vec(&e.age).unwrap()
    );
}
