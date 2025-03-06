SELECT
    id.id() as id,
    (->rel_account_roles->core_roles.id).map(|$item| $item.id()) as roles
FROM ONLY core_accounts
WHERE
    ->rel_account_refresh_tokens->(core_refresh_tokens WHERE refresh_token == <uuid> $refresh_token)
LIMIT 1;