/******************************************************************************\
*
* Admin Interface
*
\******************************************************************************/
/// Sets the administrator to the specified address `new_admin`.
///
/// # Arguments
///
/// * `new_admin` - The address which will henceforth be the administrator
///   of this token contract.
///
/// # Events
///
/// Emits an event with topics `["set_admin", admin: Address, sep0011_asset: String], data =
/// [new_admin: Address]`
fn set_admin(env: Env, new_admin: Address);

/// Returns the admin of the contract.
///
/// # Panics
///
/// If the admin is not set.
fn admin(env: Env) -> Address;

/// Sets whether the account is authorized to use its balance. If
/// `authorized` is true, `id` should be able to use its balance.
///
/// # Arguments
///
/// * `id` - The address being (de-)authorized.
/// * `authorize` - Whether or not `id` can use its balance.
///
/// # Events
///
/// Emits an event with topics `["set_authorized", id: Address, sep0011_asset: String], data =
/// [authorize: bool]`
fn set_authorized(env: Env, id: Address, authorize: bool);

/// Mints `amount` to `to`.
///
/// # Arguments
///
/// * `to` - The address which will receive the minted tokens.
/// * `amount` - The amount of tokens to be minted.
///
/// # Events
///
/// Emits an event with topics `["mint", admin: Address, to: Address, sep0011_asset: String], data
/// = amount: i128`
fn mint(env: Env, to: Address, amount: i128);

/// Clawback `amount` from `from` account. `amount` is burned in the
/// clawback process.
///
/// # Arguments
///
/// * `from` - The address holding the balance from which the clawback will
///   take tokens.
/// * `amount` - The amount of tokens to be clawed back.
///
/// # Events
///
/// Emits an event with topics `["clawback", admin: Address, to: Address, sep0011_asset: String],
/// data = amount: i128`
fn clawback(env: Env, from: Address, amount: i128);