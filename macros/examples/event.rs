use abcf::Event;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct E {
    #[abcf(index)]
    age: u8,
    name: String,
    height: u8,
    #[abcf(index)]
    weight: u8,
}

fn main() {
    let e = E {
        age: 30,
        name: "jack".to_string(),
        height: 190,
        weight: 78,
    };

    if let Ok(abci_event) = e.to_abci_event() {
        assert_eq!(e.name(), "E");
        assert!(abci_event.attributes[0].index);
        assert!(abci_event.attributes[3].index);

        assert_eq!(
            abci_event.attributes[0].key,
            String::from("age")
        );

        assert_eq!(
            abci_event.attributes[0].value,
            serde_json::to_string(&e.age).unwrap()
        );
    }
}
