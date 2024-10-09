#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, BytesN, Env, Map, String, Symbol, Vec, log};

#[contracttype]
#[derive(Debug, Clone)]
pub struct Report {
    pub subject: String,
    pub date_of_incident: String,
    pub location: String,
    pub date_of_submission: String,
    pub description_cid: String,
    pub attachments_cid: String,
    pub attachment_count: u32,
}

#[contracttype]
#[derive(Debug, Clone)]
pub struct ReportUpdate {
    pub report_secret_key: Symbol,
    pub content_cid: String,
    pub status: String,
    pub date_of_submission: String,
}

#[contracttype]
pub struct UpdateCount {
    pub count: u32,
}

#[contracttype]
pub enum UpdateCountRegistry {
    UpdateCount(Symbol),
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
}

const REPORT_KEYS:Symbol = symbol_short!("RPT_KEYS"); // report keys
const UPDATE_KEYS:Symbol = symbol_short!("RUPD_KEYS"); // report update keys
const REPORT_COUNT:Symbol = symbol_short!("RPT_COUNT"); // report count

#[contract]
pub struct ReportContract;

#[contractimpl]
impl ReportContract {
    // Update a Report
    pub fn upload_report(
        env: Env,
        secret_key: Symbol,
        subject: String,
        date_of_incident: String,
        location: String,
        date_of_submission: String,
        description_cid: String,
        attachments_cid: String,
        attachment_count: u32,
        initial_update_key: String,
    ) {
        let report = Report {
            subject,
            date_of_incident,
            location,
            description_cid,
            attachments_cid,
            date_of_submission: date_of_submission.clone(),
            attachment_count,
        };

        let mut report_count: u64 = env.storage().persistent().get(&REPORT_COUNT).unwrap_or(0);

        env.storage().persistent().set(&secret_key, &report);

        report_count += 1;

        // Save the key to the report keys list
        let mut report_keys: Vec<Symbol> = env
            .storage()
            .persistent()
            .get(&REPORT_KEYS)
            .unwrap_or(Vec::new(&env));

        report_keys.push_back(secret_key.clone());

        env.storage().persistent().set(&REPORT_KEYS, &report_keys);
        env.storage().persistent().set(&REPORT_COUNT, &report_count);

        // set default report update status to "submitted"
        Self::upload_report_update(
            env.clone(),
            secret_key,
            initial_update_key,
            String::from_str(&env, "__NULL__"),
            String::from_str(&env, "submitted"),
            date_of_submission,
        );
    }

    // Fetch a Report
    pub fn fetch_report(env: Env, secret_key: Symbol) -> Option<Report> {
        env.storage().persistent().get(&secret_key)
    }

    // Fetch all Reports
    pub fn fetch_all_reports(env: Env) -> Map<Symbol, Report> {
        let mut reports = Map::new(&env);

        let report_keys: Vec<Symbol> = env
            .storage()
            .persistent()
            .get(&REPORT_KEYS)
            .unwrap_or(Vec::new(&env));

        for key in report_keys.iter() {
            if let Some(report) = Self::fetch_report(env.clone(), key.clone()) {
                reports.set(key, report);
            }
        }

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "Reports: {}", reports);

        reports
    }

    pub fn get_report_count(env: Env) -> u64 {
        env.storage().persistent().get(&REPORT_COUNT).unwrap_or(0)
    }

    pub fn get_all_report_keys(env: Env) -> Vec<Symbol> {
        env.storage()
            .persistent()
            .get(&REPORT_KEYS)
            .unwrap_or(Vec::new(&env))
    }

    // Upload a Report Update
    pub fn upload_report_update(
        env: Env,
        report_secret_key: Symbol,
        update_key: String,
        content_cid: String,
        status: String,
        date_of_submission: String,
    ) {
        let mut update_count =
            Self::get_report_update_count(env.clone(), report_secret_key.clone());

        let report_update = ReportUpdate {
            report_secret_key: report_secret_key.clone(),
            content_cid,
            status,
            date_of_submission,
        };

        env.storage().persistent().set(&update_key, &report_update);

        update_count.count = update_count.count + 1;

        env.storage().persistent().set(
            &UpdateCountRegistry::UpdateCount(report_secret_key),
            &update_count,
        );

        // Save the update key to the report update keys list
        let mut update_keys: Vec<String> = env
            .storage()
            .persistent()
            .get(&UPDATE_KEYS)
            .unwrap_or(Vec::new(&env));

        update_keys.push_back(update_key);
        env.storage().persistent().set(&UPDATE_KEYS, &update_keys);
    }

    // Fetch a Report Update
    pub fn fetch_report_update(env: Env, update_key: String) -> Option<ReportUpdate> {
        env.storage().persistent().get(&update_key)
    }

    // Fetch all Report Updates for a Specific Report
    pub fn fetch_all_report_updates(env: Env, report_secret_key: Symbol) -> Vec<ReportUpdate> {
        let mut updates = Vec::new(&env);

        let update_keys: Vec<String> = env
            .storage()
            .persistent()
            .get(&UPDATE_KEYS)
            .unwrap_or(Vec::new(&env));

        for key in update_keys.iter() {
            if let Some(update) = env.storage().persistent().get::<String, ReportUpdate>(&key) {
                if update.report_secret_key == report_secret_key {
                    updates.push_back(update);
                }
            }
        }

        /// @dev should remove logs before deploying smart contracts
        log!(&env, "Updates: {}", updates);

        updates
    }

    // Get Report Update Count
    pub fn get_report_update_count(env: Env, report_secret_key: Symbol) -> UpdateCount {
        let key = UpdateCountRegistry::UpdateCount(report_secret_key);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(UpdateCount {count: 0})
    }

    // Logic for Contract Upgrade

    pub fn init(e: Env, admin: Address) {
        e.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn version() -> u32 {
        1
    }

    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        let admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
