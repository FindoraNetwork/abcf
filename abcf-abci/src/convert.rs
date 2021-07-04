use abcf_core::message;
use async_abci::abci;

pub trait Convert {
    type Target;

    fn convert(self) -> Self::Target;
}

macro_rules! define_type_convert {
    ($for_type:path, $target_type:path, $($key:ident),*) => {
        impl Convert for $for_type {
            type Target = $target_type;

            fn convert(self) -> Self::Target {
                Self::Target {
                    $( $key: self.$key, )*
                }
            }
        }
    }
}

define_type_convert!(abci::RequestEcho, message::echo::Request, message);
define_type_convert!(
    abci::RequestInfo,
    message::info::Request,
    version,
    block_version,
    p2p_version
);

// impl Convert for abci::RequestInitChain {
// type Target = message::init_chain::Request;
//
// fn convert(self) -> Self::Target {
//     let req = message::init_chain::Request::default();
//     req
// }
// }

define_type_convert!(
    abci::RequestQuery,
    message::query::Request,
    data,
    path,
    height,
    prove
);

define_type_convert!(message::echo::Response, abci::ResponseEcho, message);
define_type_convert!(
    message::info::Response,
    abci::ResponseInfo,
    data,
    version,
    app_version,
    last_block_height,
    last_block_app_hash
);
