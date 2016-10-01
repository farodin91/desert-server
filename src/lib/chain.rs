
pub enum ResultData {
    Futures,
    Blocking,
}

use lib::error::Result;

pub trait Chain {
    fn run(&self) -> Result<ResultData>;
}

pub trait ChainLink {
    fn name(&self) -> String;
    fn link<T,E>(&self, input: T) -> Result<E>;
}

macro_rules! chain {
    (
        $name:ident ($ty:ident) {
            $($Type:ty,)+
        }
    ) => {
        chain_body! {
            chain_name = $name,
            error_ty = $ty,
            links = [$($Type,)+],
        }
    };
}

macro_rules! chain_body {
    (
        chain_name = $chain_name:ident,
        error_ty = $error_ty:ty,
        links = [$($link_ty:ty,)+],
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $chain_name;

        impl Chain for $chain_name {
            fn run(&self) -> Result<ResultData> {
                Ok(ResultData::Futures)
            }
        }
    }
}

pub struct Test;

chain! {
  Index(ReturnError) {
    JsonRequest<Test>,
    AccessTokenAuth,
    Processing,
  }
}