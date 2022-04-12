// instruction.rs -> program API, (de)serializing instruction data

pub enum DespositInstruction {


    /// Starts the trade by creating and populating a deposit account and transferring ownership of the given temp token account to the PDA
   ///
   ///
   /// Accounts expected:
   ///
   /// 0. `[signer]` The account of the person initializing the deposit process
   /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
   /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
   /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
   /// 4. `[]` The rent sysvar
   /// 5. `[]` The token program


   InitDeposit{
       //The amount party A suppose to deposit
       amount: u64
   }
}

