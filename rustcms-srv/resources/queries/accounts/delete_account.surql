BEGIN TRANSACTION;

LET $account_rec = type::thing("core_accounts", <string> $account_id);

DELETE $account_rec->rel_account_refresh_tokens->core_refresh_tokens;
DELETE $account_rec;

COMMIT TRANSACTION;