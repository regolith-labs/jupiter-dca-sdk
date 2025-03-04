//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitiateFlashFill {
    pub keeper: solana_program::pubkey::Pubkey,

    pub dca: solana_program::pubkey::Pubkey,
    /// The token to borrow
    pub input_mint: solana_program::pubkey::Pubkey,
    /// The account to send borrowed tokens to
    pub keeper_in_ata: solana_program::pubkey::Pubkey,
    /// The account to borrow from
    pub in_ata: solana_program::pubkey::Pubkey,
    /// The account to repay to
    pub out_ata: solana_program::pubkey::Pubkey,
    /// Solana Instructions Sysvar
    pub instructions_sysvar: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,
}

impl InitiateFlashFill {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.keeper,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.dca, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.input_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.keeper_in_ata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.in_ata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.out_ata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.instructions_sysvar,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = InitiateFlashFillInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::DCA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitiateFlashFillInstructionData {
    discriminator: [u8; 8],
}

impl InitiateFlashFillInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [143, 205, 3, 191, 162, 215, 245, 49],
        }
    }
}

impl Default for InitiateFlashFillInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `InitiateFlashFill`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` keeper
///   1. `[writable]` dca
///   2. `[]` input_mint
///   3. `[writable]` keeper_in_ata
///   4. `[writable]` in_ata
///   5. `[]` out_ata
///   6. `[optional]` instructions_sysvar (default to `Sysvar1nstructions1111111111111111111111111`)
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   9. `[]` associated_token_program
#[derive(Clone, Debug, Default)]
pub struct InitiateFlashFillBuilder {
    keeper: Option<solana_program::pubkey::Pubkey>,
    dca: Option<solana_program::pubkey::Pubkey>,
    input_mint: Option<solana_program::pubkey::Pubkey>,
    keeper_in_ata: Option<solana_program::pubkey::Pubkey>,
    in_ata: Option<solana_program::pubkey::Pubkey>,
    out_ata: Option<solana_program::pubkey::Pubkey>,
    instructions_sysvar: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitiateFlashFillBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn keeper(&mut self, keeper: solana_program::pubkey::Pubkey) -> &mut Self {
        self.keeper = Some(keeper);
        self
    }
    #[inline(always)]
    pub fn dca(&mut self, dca: solana_program::pubkey::Pubkey) -> &mut Self {
        self.dca = Some(dca);
        self
    }
    /// The token to borrow
    #[inline(always)]
    pub fn input_mint(&mut self, input_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.input_mint = Some(input_mint);
        self
    }
    /// The account to send borrowed tokens to
    #[inline(always)]
    pub fn keeper_in_ata(&mut self, keeper_in_ata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.keeper_in_ata = Some(keeper_in_ata);
        self
    }
    /// The account to borrow from
    #[inline(always)]
    pub fn in_ata(&mut self, in_ata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.in_ata = Some(in_ata);
        self
    }
    /// The account to repay to
    #[inline(always)]
    pub fn out_ata(&mut self, out_ata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.out_ata = Some(out_ata);
        self
    }
    /// `[optional account, default to 'Sysvar1nstructions1111111111111111111111111']`
    /// Solana Instructions Sysvar
    #[inline(always)]
    pub fn instructions_sysvar(
        &mut self,
        instructions_sysvar: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.instructions_sysvar = Some(instructions_sysvar);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = InitiateFlashFill {
            keeper: self.keeper.expect("keeper is not set"),
            dca: self.dca.expect("dca is not set"),
            input_mint: self.input_mint.expect("input_mint is not set"),
            keeper_in_ata: self.keeper_in_ata.expect("keeper_in_ata is not set"),
            in_ata: self.in_ata.expect("in_ata is not set"),
            out_ata: self.out_ata.expect("out_ata is not set"),
            instructions_sysvar: self.instructions_sysvar.unwrap_or(solana_program::pubkey!(
                "Sysvar1nstructions1111111111111111111111111"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            associated_token_program: self
                .associated_token_program
                .expect("associated_token_program is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `initiate_flash_fill` CPI accounts.
pub struct InitiateFlashFillCpiAccounts<'a, 'b> {
    pub keeper: &'b solana_program::account_info::AccountInfo<'a>,

    pub dca: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token to borrow
    pub input_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to send borrowed tokens to
    pub keeper_in_ata: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to borrow from
    pub in_ata: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to repay to
    pub out_ata: &'b solana_program::account_info::AccountInfo<'a>,
    /// Solana Instructions Sysvar
    pub instructions_sysvar: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initiate_flash_fill` CPI instruction.
pub struct InitiateFlashFillCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub keeper: &'b solana_program::account_info::AccountInfo<'a>,

    pub dca: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token to borrow
    pub input_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to send borrowed tokens to
    pub keeper_in_ata: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to borrow from
    pub in_ata: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to repay to
    pub out_ata: &'b solana_program::account_info::AccountInfo<'a>,
    /// Solana Instructions Sysvar
    pub instructions_sysvar: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> InitiateFlashFillCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitiateFlashFillCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            keeper: accounts.keeper,
            dca: accounts.dca,
            input_mint: accounts.input_mint,
            keeper_in_ata: accounts.keeper_in_ata,
            in_ata: accounts.in_ata,
            out_ata: accounts.out_ata,
            instructions_sysvar: accounts.instructions_sysvar,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            associated_token_program: accounts.associated_token_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.keeper.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.dca.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.input_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.keeper_in_ata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.in_ata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.out_ata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.instructions_sysvar.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = InitiateFlashFillInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DCA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.keeper.clone());
        account_infos.push(self.dca.clone());
        account_infos.push(self.input_mint.clone());
        account_infos.push(self.keeper_in_ata.clone());
        account_infos.push(self.in_ata.clone());
        account_infos.push(self.out_ata.clone());
        account_infos.push(self.instructions_sysvar.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.associated_token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `InitiateFlashFill` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` keeper
///   1. `[writable]` dca
///   2. `[]` input_mint
///   3. `[writable]` keeper_in_ata
///   4. `[writable]` in_ata
///   5. `[]` out_ata
///   6. `[]` instructions_sysvar
///   7. `[]` system_program
///   8. `[]` token_program
///   9. `[]` associated_token_program
#[derive(Clone, Debug)]
pub struct InitiateFlashFillCpiBuilder<'a, 'b> {
    instruction: Box<InitiateFlashFillCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitiateFlashFillCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitiateFlashFillCpiBuilderInstruction {
            __program: program,
            keeper: None,
            dca: None,
            input_mint: None,
            keeper_in_ata: None,
            in_ata: None,
            out_ata: None,
            instructions_sysvar: None,
            system_program: None,
            token_program: None,
            associated_token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn keeper(
        &mut self,
        keeper: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.keeper = Some(keeper);
        self
    }
    #[inline(always)]
    pub fn dca(&mut self, dca: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.dca = Some(dca);
        self
    }
    /// The token to borrow
    #[inline(always)]
    pub fn input_mint(
        &mut self,
        input_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.input_mint = Some(input_mint);
        self
    }
    /// The account to send borrowed tokens to
    #[inline(always)]
    pub fn keeper_in_ata(
        &mut self,
        keeper_in_ata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.keeper_in_ata = Some(keeper_in_ata);
        self
    }
    /// The account to borrow from
    #[inline(always)]
    pub fn in_ata(
        &mut self,
        in_ata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.in_ata = Some(in_ata);
        self
    }
    /// The account to repay to
    #[inline(always)]
    pub fn out_ata(
        &mut self,
        out_ata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.out_ata = Some(out_ata);
        self
    }
    /// Solana Instructions Sysvar
    #[inline(always)]
    pub fn instructions_sysvar(
        &mut self,
        instructions_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.instructions_sysvar = Some(instructions_sysvar);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = InitiateFlashFillCpi {
            __program: self.instruction.__program,

            keeper: self.instruction.keeper.expect("keeper is not set"),

            dca: self.instruction.dca.expect("dca is not set"),

            input_mint: self.instruction.input_mint.expect("input_mint is not set"),

            keeper_in_ata: self
                .instruction
                .keeper_in_ata
                .expect("keeper_in_ata is not set"),

            in_ata: self.instruction.in_ata.expect("in_ata is not set"),

            out_ata: self.instruction.out_ata.expect("out_ata is not set"),

            instructions_sysvar: self
                .instruction
                .instructions_sysvar
                .expect("instructions_sysvar is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitiateFlashFillCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    keeper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    dca: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    input_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    keeper_in_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    in_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    out_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    instructions_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
