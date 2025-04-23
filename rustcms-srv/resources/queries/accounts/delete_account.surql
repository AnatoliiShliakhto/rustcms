BEGIN TRANSACTION;

LET $account_rec = type::thing('core_accounts', $account_id);

IF $account_rec.exists() THEN {
    DELETE $account_rec->rel_account_refresh_tokens->core_refresh_tokens;
    DELETE $account_rec;
} END;

RETURN true;

COMMIT TRANSACTION;