BEGIN TRANSACTION;

LET $refresh_token_obj = CREATE ONLY core_refresh_tokens CONTENT {
    expiration_at: $expiration_at,
    device: $device
} RETURN id, refresh_token;

LET $account_rec = type::thing('core_accounts', $account_id);
LET $token_rec = $refresh_token_obj.id;

RELATE $account_rec->rel_account_refresh_tokens->$token_rec;

RETURN <string> $refresh_token_obj.refresh_token;

COMMIT TRANSACTION;