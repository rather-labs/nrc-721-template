use ckb_std::{
    ckb_types::{
        bytes::Bytes, 
        packed::*, 
        prelude::*
    },
    high_level::load_script,
};
use core::result::Result;
use nrc_721::*;
use nrc_721::error::*;
use nrc_721::helper::*;

///////////////////////////////////////////////////////////////////////////////
// Custom behavior

pub struct Custom;

impl Custom {
    pub fn handle_creation(_nft_type: &Script) -> Result<(), Error> {
        // if we found some invalid condition
        // Err(Error::NFTDataInvalid)
        Ok(())
    }

    pub fn handle_update(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }

    pub fn handle_destroying(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }
}

// Composite script

use nrc_721::extensions::OnlyOwner;

define_script! { ComposedScript(Base, OnlyOwner, Custom) { } }

///////////////////////////////////////////////////////////////////////////////

pub fn main() -> Result<(), Error> {
    let nft_type = load_script()?;
    ComposedScript::validate_nft_args(&nft_type)?;
    match nrc_721::parse_nft_action(&nft_type)? {
        Action::Create => ComposedScript::handle_creation(&nft_type),
        Action::Update => ComposedScript::handle_update(&nft_type),
        Action::Destroy => ComposedScript::handle_destroying(&nft_type),
    }
}