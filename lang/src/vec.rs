use crate::{Accounts, ToAccountInfos, ToAccountMetas};
use solana_program::account_info::AccountInfo;
use solana_program::instruction::AccountMeta;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

impl<'info, T: ToAccountInfos<'info>> ToAccountInfos<'info> for Vec<T> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        self.iter()
            .flat_map(|item| item.to_account_infos())
            .collect()
    }
}

impl<T: ToAccountMetas> ToAccountMetas for Vec<T> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        self.iter()
            .flat_map(|item| (*item).to_account_metas(is_signer))
            .collect()
    }
}

impl<'info, T: Accounts<'info>> Accounts<'info> for Vec<T> {
    fn try_accounts(
        program_id: &Pubkey,
        accounts: &mut &[AccountInfo<'info>],
    ) -> Result<Self, ProgramError> {
        if accounts.is_empty() {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let ret =
            accounts.iter()
                    .flat_map(|item| T::try_accounts(&PubKey, &mut &[*item]))//fix
                    .collect();
        *accounts = &[];
        return ret;
    }
}